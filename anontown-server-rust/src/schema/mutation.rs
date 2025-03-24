use juniper::{graphql_object, graphql_value, FieldError, FieldResult, ID};
use crate::ports::Ports;
use crate::entities::{User, TokenMaster, Client, TokenGeneral, TopicNormal, TopicOne, TopicFork, TopicEdit, Res, Profile, Storage};
use crate::schema::types::{
    ClientType, CreateClientInput, CreateTokenInput, CreateUserInput, HistoryType, ProfileType,
    ResType, StorageType, TagType, TokenType, TopicType, UpdateClientInput, UpdateTokenInput,
    UpdateUserInput, UserType, CreateTokenGeneralResponse, TokenReq, SetStoragesInput, SetStoragesPayload,
};
use crate::schema::input::{
    CreateResInput, CreateTopicNormalInput, CreateTopicOneInput,
    CreateTopicForkInput, UpdateTopicInput,
};
use crate::ports::{ResPort, TopicPort, UserPort};

pub struct Mutation;

#[graphql_object]
impl Mutation {
    async fn create_user(&self, context: &Context, input: CreateUserInput) -> FieldResult<UserType> {
        // reCAPTCHAの検証
        context.ports.recaptcha.verify(&input.recaptcha).await?;

        // ユーザーの作成
        let user = User::create(
            &context.ports.object_id_generator,
            &input.sn,
            &input.pass,
            context.ports.clock.now(),
        );

        // ユーザーの保存
        context.ports.user_repo.insert(&user).await?;

        // マスタートークンの作成
        let token = TokenMaster::create(
            &context.ports.object_id_generator,
            user.auth(&input.pass),
            context.ports.clock.now(),
            &context.ports.safe_id_generator,
        );

        // トークンの保存
        context.ports.token_repo.insert(&token).await?;

        Ok(UserType::from(user))
    }

    async fn update_user(&self, context: &Context, input: UpdateUserInput) -> FieldResult<UserType> {
        // 認証ユーザーの取得
        let auth_user = context.ports.auth_from_api_param.auth_user_request_to_user(
            &context.ports.user_repo,
            &input.auth,
        ).await?;

        // ユーザーの取得
        let user = context.ports.user_repo.find_one(&auth_user.id).await?;

        // ユーザーの更新
        let new_user = user.change(
            auth_user,
            input.pass.as_deref(),
            input.sn.as_deref(),
        );

        // ユーザーの保存
        context.ports.user_repo.update(&new_user).await?;

        // マスタートークンの削除
        context.ports.token_repo.del_master_token(&auth_user).await?;

        // 新しいマスタートークンの作成
        let token = TokenMaster::create(
            &context.ports.object_id_generator,
            auth_user,
            context.ports.clock.now(),
            &context.ports.safe_id_generator,
        );

        // トークンの保存
        context.ports.token_repo.insert(&token).await?;

        Ok(UserType::from(new_user))
    }

    async fn create_client(&self, context: &Context, input: CreateClientInput) -> FieldResult<ClientType> {
        // クライアントの作成
        let client = Client::create(
            &context.ports.object_id_generator,
            context.ports.auth_container.get_token_master(),
            &input.name,
            &input.url,
            context.ports.clock.now(),
        );

        // クライアントの保存
        context.ports.client_repo.insert(&client).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: clients {}",
                client.id
            )
        );

        Ok(ClientType::from(client))
    }

    async fn update_client(&self, context: &Context, input: UpdateClientInput) -> FieldResult<ClientType> {
        // クライアントの取得
        let client = context.ports.client_repo.find_one(&input.id).await?;

        // クライアントの更新
        let new_client = client.change_data(
            context.ports.auth_container.get_token_master(),
            input.name.as_deref(),
            input.url.as_deref(),
            context.ports.clock.now(),
        );

        // クライアントの保存
        context.ports.client_repo.update(&new_client).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: clients {}",
                client.id
            )
        );

        Ok(ClientType::from(new_client))
    }

    async fn create_token_general(&self, context: &Context, client: ID) -> FieldResult<CreateTokenGeneralResponse> {
        // クライアントの取得
        let client = context.ports.client_repo.find_one(&client).await?;

        // トークンの作成
        let token = TokenGeneral::create(
            &context.ports.object_id_generator,
            context.ports.auth_container.get_token_master(),
            &client,
            context.ports.clock.now(),
            &context.ports.safe_id_generator,
        );

        // トークンリクエストの作成
        let (req, new_token) = token.create_req(
            context.ports.clock.now(),
            &context.ports.safe_id_generator,
        );

        // トークンの保存
        context.ports.token_repo.insert(&new_token).await?;

        Ok(CreateTokenGeneralResponse {
            token: TokenType::from(token),
            req: TokenReq {
                token: req.token,
                key: req.key,
            },
        })
    }

    async fn create_token_req(&self, context: &Context) -> FieldResult<TokenReq> {
        // トークンリクエストの作成
        let req = TokenReq::create(
            context.ports.clock.now(),
            &context.ports.safe_id_generator,
        );

        Ok(req)
    }

    async fn create_token_master(&self, context: &Context, auth: AuthUser) -> FieldResult<TokenType> {
        // 認証ユーザーの取得
        let auth_user = context.ports.auth_from_api_param.auth_user_request_to_user(
            &context.ports.user_repo,
            &auth,
        ).await?;

        // マスタートークンの作成
        let token = TokenMaster::create(
            &context.ports.object_id_generator,
            auth_user,
            context.ports.clock.now(),
            &context.ports.safe_id_generator,
        );

        // トークンの保存
        context.ports.token_repo.insert(&token).await?;

        Ok(TokenType::from(token))
    }

    async fn auth_token_req(&self, context: &Context, id: ID, key: String) -> FieldResult<TokenType> {
        // トークンの認証
        let token = context.ports.token_repo.auth_token_req(&id, &key).await?;

        Ok(TokenType::from(token))
    }

    async fn del_token_client(&self, context: &Context, client: ID) -> FieldResult<bool> {
        // クライアントの取得
        let client = context.ports.client_repo.find_one(&client).await?;

        // クライアントトークンの削除
        context.ports.token_repo.del_client_token(
            context.ports.auth_container.get_token_master(),
            &client.id,
        ).await?;

        Ok(true)
    }

    async fn create_topic_normal(&self, context: &Context, title: String, tags: Vec<String>, text: String) -> FieldResult<TopicType> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // トピックの作成
        let create = TopicNormal::create(
            &context.ports.object_id_generator,
            &title,
            &tags,
            &text,
            &user,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        );

        // トピックの保存
        context.ports.topic_repo.insert(&create.topic).await?;

        // ユーザー、レス、履歴の保存
        context.ports.user_repo.update(&create.user).await?;
        context.ports.res_repo.insert(&create.res).await?;
        context.ports.history_repo.insert(&create.history).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: topics {}",
                create.topic.id
            )
        );
        context.ports.logger.info(
            format!(
                "mutation: reses {}",
                create.res.id
            )
        );
        context.ports.logger.info(
            format!(
                "mutation: histories {}",
                create.history.id
            )
        );

        Ok(TopicType::from(create.topic))
    }

    async fn create_topic_one(&self, context: &Context, title: String, text: String) -> FieldResult<TopicType> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // トピックの作成
        let create = TopicOne::create(
            &context.ports.object_id_generator,
            &title,
            &text,
            &user,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        );

        // トピックの保存
        context.ports.topic_repo.insert(&create.topic).await?;

        // ユーザー、レス、履歴の保存
        context.ports.user_repo.update(&create.user).await?;
        context.ports.res_repo.insert(&create.res).await?;
        context.ports.history_repo.insert(&create.history).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: topics {}",
                create.topic.id
            )
        );
        context.ports.logger.info(
            format!(
                "mutation: reses {}",
                create.res.id
            )
        );
        context.ports.logger.info(
            format!(
                "mutation: histories {}",
                create.history.id
            )
        );

        Ok(TopicType::from(create.topic))
    }

    async fn create_topic_fork(&self, context: &Context, title: String, text: String, parent: ID) -> FieldResult<TopicType> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // 親トピックの取得
        let parent = context.ports.topic_repo.find_one(&parent).await?;

        // トピックの作成
        let create = TopicFork::create(
            &context.ports.object_id_generator,
            &title,
            &text,
            &user,
            &parent,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        );

        // トピックの保存
        context.ports.topic_repo.insert(&create.topic).await?;

        // ユーザー、レス、履歴の保存
        context.ports.user_repo.update(&create.user).await?;
        context.ports.res_repo.insert(&create.res).await?;
        context.ports.history_repo.insert(&create.history).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: topics {}",
                create.topic.id
            )
        );
        context.ports.logger.info(
            format!(
                "mutation: reses {}",
                create.res.id
            )
        );
        context.ports.logger.info(
            format!(
                "mutation: histories {}",
                create.history.id
            )
        );

        Ok(TopicType::from(create.topic))
    }

    async fn create_topic_edit(&self, context: &Context, title: String, text: String, parent: ID) -> FieldResult<TopicType> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // 親トピックの取得
        let parent = context.ports.topic_repo.find_one(&parent).await?;

        // トピックの作成
        let create = TopicEdit::create(
            &context.ports.object_id_generator,
            &title,
            &text,
            &user,
            &parent,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        );

        // トピックの保存
        context.ports.topic_repo.insert(&create.topic).await?;

        // ユーザー、レス、履歴の保存
        context.ports.user_repo.update(&create.user).await?;
        context.ports.res_repo.insert(&create.res).await?;
        context.ports.history_repo.insert(&create.history).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: topics {}",
                create.topic.id
            )
        );
        context.ports.logger.info(
            format!(
                "mutation: reses {}",
                create.res.id
            )
        );
        context.ports.logger.info(
            format!(
                "mutation: histories {}",
                create.history.id
            )
        );

        Ok(TopicType::from(create.topic))
    }

    async fn update_topic(&self, context: &Context, id: ID, title: Option<String>, tags: Option<Vec<String>>, text: Option<String>) -> FieldResult<TopicType> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // トピックの取得
        let topic = context.ports.topic_repo.find_one(&id).await?;

        // トピックの更新
        let update = topic.update(
            &user,
            title.as_deref(),
            tags.as_deref(),
            text.as_deref(),
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        )?;

        // トピックの保存
        context.ports.topic_repo.update(&update.topic).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: topics {}",
                update.topic.id
            )
        );

        Ok(TopicType::from(update.topic))
    }

    async fn create_res(&self, context: &Context, text: String, topic: ID) -> FieldResult<ResType> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // トピックの取得
        let topic = context.ports.topic_repo.find_one(&topic).await?;

        // レスの作成
        let create = Res::create(
            &context.ports.object_id_generator,
            &text,
            &user,
            &topic,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        );

        // レスの保存
        context.ports.res_repo.insert(&create.res).await?;

        // ユーザー、履歴の保存
        context.ports.user_repo.update(&create.user).await?;
        context.ports.history_repo.insert(&create.history).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: reses {}",
                create.res.id
            )
        );
        context.ports.logger.info(
            format!(
                "mutation: histories {}",
                create.history.id
            )
        );

        Ok(ResType::from(create.res))
    }

    async fn vote_res(&self, context: &Context, res: ID) -> FieldResult<ResType> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // レスの取得
        let res = context.ports.res_repo.find_one(&res).await?;

        // レスの投票
        let vote = res.vote(
            &user,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        );

        // レスの保存
        context.ports.res_repo.update(&vote.res).await?;

        // ユーザーの保存
        context.ports.user_repo.update(&vote.user).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: reses {}",
                vote.res.id
            )
        );

        Ok(ResType::from(vote.res))
    }

    async fn del_res(&self, context: &Context, res: ID) -> FieldResult<bool> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // レスの取得
        let res = context.ports.res_repo.find_one(&res).await?;

        // レスの削除
        res.del(
            &user,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        )?;

        // レスの保存
        context.ports.res_repo.update(&res).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: reses {}",
                res.id
            )
        );

        Ok(true)
    }

    async fn create_profile(&self, context: &Context, name: String, text: String) -> FieldResult<ProfileType> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // プロフィールの作成
        let create = Profile::create(
            &context.ports.object_id_generator,
            &name,
            &text,
            &user,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        );

        // プロフィールの保存
        context.ports.profile_repo.insert(&create.profile).await?;

        // ユーザーの保存
        context.ports.user_repo.update(&create.user).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: profiles {}",
                create.profile.id
            )
        );

        Ok(ProfileType::from(create.profile))
    }

    async fn update_profile(&self, context: &Context, id: ID, name: String, text: String) -> FieldResult<ProfileType> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // プロフィールの取得
        let profile = context.ports.profile_repo.find_one(&id).await?;

        // プロフィールの更新
        let update = profile.update(
            &user,
            &name,
            &text,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        )?;

        // プロフィールの保存
        context.ports.profile_repo.update(&update.profile).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: profiles {}",
                update.profile.id
            )
        );

        Ok(ProfileType::from(update.profile))
    }

    async fn del_profile(&self, context: &Context, id: ID) -> FieldResult<bool> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // プロフィールの取得
        let profile = context.ports.profile_repo.find_one(&id).await?;

        // プロフィールの削除
        profile.del(
            &user,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        )?;

        // プロフィールの保存
        context.ports.profile_repo.update(&profile).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: profiles {}",
                profile.id
            )
        );

        Ok(true)
    }

    async fn create_storage(&self, context: &Context, key: String, value: String) -> FieldResult<StorageType> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // ストレージの作成
        let create = Storage::create(
            &context.ports.object_id_generator,
            &key,
            &value,
            &user,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        );

        // ストレージの保存
        context.ports.storage_repo.insert(&create.storage).await?;

        // ユーザーの保存
        context.ports.user_repo.update(&create.user).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: storages {}",
                create.storage.id
            )
        );

        Ok(StorageType::from(create.storage))
    }

    async fn update_storage(&self, context: &Context, id: ID, value: String) -> FieldResult<StorageType> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // ストレージの取得
        let storage = context.ports.storage_repo.find_one(&id).await?;

        // ストレージの更新
        let update = storage.update(
            &user,
            &value,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        )?;

        // ストレージの保存
        context.ports.storage_repo.update(&update.storage).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: storages {}",
                update.storage.id
            )
        );

        Ok(StorageType::from(update.storage))
    }

    async fn del_storage(&self, context: &Context, id: ID) -> FieldResult<bool> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // ストレージの取得
        let storage = context.ports.storage_repo.find_one(&id).await?;

        // ストレージの削除
        storage.del(
            &user,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        )?;

        // ストレージの保存
        context.ports.storage_repo.update(&storage).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: storages {}",
                storage.id
            )
        );

        Ok(true)
    }

    async fn set_storages(&self, context: &Context, input: SetStoragesInput) -> FieldResult<SetStoragesPayload> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // ストレージの設定
        let storages = context.ports.storage_repo.set_storages(
            &user,
            &input.storages,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        ).await?;

        // ログの出力
        for storage in &storages {
            context.ports.logger.info(
                format!(
                    "mutation: storages {}",
                    storage.id
                )
            );
        }

        Ok(SetStoragesPayload {
            storages: storages.into_iter().map(StorageType::from).collect(),
        })
    }

    async fn subscribe_topic(&self, context: &Context, topic: ID) -> FieldResult<bool> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // トピックの取得
        let topic = context.ports.topic_repo.find_one(&topic).await?;

        // トピックの購読
        let subscribe = topic.subscribe(
            &user,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        )?;

        // トピックの保存
        context.ports.topic_repo.update(&subscribe.topic).await?;

        // ユーザーの保存
        context.ports.user_repo.update(&subscribe.user).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: topics {}",
                subscribe.topic.id
            )
        );

        Ok(true)
    }

    async fn unsubscribe_topic(&self, context: &Context, topic: ID) -> FieldResult<bool> {
        // ユーザーの取得
        let user = context.ports.user_repo.find_one(
            context.ports.auth_container.get_token().user,
        ).await?;

        // トピックの取得
        let topic = context.ports.topic_repo.find_one(&topic).await?;

        // トピックの購読解除
        let unsubscribe = topic.unsubscribe(
            &user,
            context.ports.auth_container.get_token(),
            context.ports.clock.now(),
        )?;

        // トピックの保存
        context.ports.topic_repo.update(&unsubscribe.topic).await?;

        // ユーザーの保存
        context.ports.user_repo.update(&unsubscribe.user).await?;

        // ログの出力
        context.ports.logger.info(
            format!(
                "mutation: topics {}",
                unsubscribe.topic.id
            )
        );

        Ok(true)
    }

    async fn resister_push_subscription(&self, context: &Context, endpoint: String, p256dh: String, auth: String) -> FieldResult<bool> {
        // プッシュ通知の購読登録
        context.ports.push_subscriptions_repo.upsert(
            context.ports.auth_container.get_token().user,
            &endpoint,
            &p256dh,
            &auth,
        ).await?;

        Ok(true)
    }

    pub async fn create_res(
        &self,
        input: CreateResInput,
        res_port: &dyn ResPort,
        user_port: &dyn UserPort,
    ) -> FieldResult<ResType> {
        let res = res_port.create(input).await?;
        Ok(ResType::from(res))
    }

    pub async fn vote_res(
        &self,
        res_id: String,
        vote_type: VoteType,
        res_port: &dyn ResPort,
        user_port: &dyn UserPort,
    ) -> FieldResult<ResType> {
        let res = res_port.vote(res_id, vote_type).await?;
        Ok(ResType::from(res))
    }

    pub async fn del_res(
        &self,
        res_id: String,
        res_port: &dyn ResPort,
        user_port: &dyn UserPort,
    ) -> FieldResult<ResType> {
        let res = res_port.delete(res_id).await?;
        Ok(ResType::from(res))
    }

    pub async fn create_topic_normal(
        &self,
        input: CreateTopicNormalInput,
        topic_port: &dyn TopicPort,
        user_port: &dyn UserPort,
    ) -> FieldResult<TopicType> {
        let topic = topic_port.create_normal(input).await?;
        Ok(TopicType::from(topic))
    }

    pub async fn create_topic_one(
        &self,
        input: CreateTopicOneInput,
        topic_port: &dyn TopicPort,
        user_port: &dyn UserPort,
    ) -> FieldResult<TopicType> {
        let topic = topic_port.create_one(input).await?;
        Ok(TopicType::from(topic))
    }

    pub async fn create_topic_fork(
        &self,
        input: CreateTopicForkInput,
        topic_port: &dyn TopicPort,
        user_port: &dyn UserPort,
    ) -> FieldResult<TopicType> {
        let topic = topic_port.create_fork(input).await?;
        Ok(TopicType::from(topic))
    }

    pub async fn update_topic(
        &self,
        input: UpdateTopicInput,
        topic_port: &dyn TopicPort,
        user_port: &dyn UserPort,
    ) -> FieldResult<TopicType> {
        let topic = topic_port.update(input).await?;
        Ok(TopicType::from(topic))
    }

    pub async fn subscribe_topic(
        &self,
        topic_id: String,
        topic_port: &dyn TopicPort,
        user_port: &dyn UserPort,
    ) -> FieldResult<bool> {
        topic_port.subscribe(topic_id).await?;
        Ok(true)
    }

    pub async fn unsubscribe_topic(
        &self,
        topic_id: String,
        topic_port: &dyn TopicPort,
        user_port: &dyn UserPort,
    ) -> FieldResult<bool> {
        topic_port.unsubscribe(topic_id).await?;
        Ok(true)
    }
} 