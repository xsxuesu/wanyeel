use crate::middleware::get_local;
use crate::APPLICATION_CONTEXT;
use async_trait::async_trait;
use wy_domain::error::Result;
use wy_domain::entity::{PageData,CommonField};

use rbatis::crud::{CRUDTable, Skip, CRUD};
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::rbatis::Rbatis;
use rbatis::wrapper::Wrapper;
use rbatis::DateTimeNative;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::From;
use std::collections::HashMap;
/**
 *struct:CrudService
 *desc:orm基础CRUD实现
 *author:String
 *email:249608904@qq.com
 */
#[async_trait]
pub trait CrudService<Entity, Dto, Params>: Sync + Send
where
    Entity: CRUDTable + DeserializeOwned + Clone,
    Dto: From<Entity> + Send + Sync + Serialize,
    Params: Send + Sync + Serialize,
{
    /**
     * 获取查询条件Wrapper
     * 子类实现
     */
    fn get_wrapper(arg: &Params) -> Wrapper;
    /**设置公共的字段保存方法*/
    fn set_save_common_fields(&self, common: CommonField, data: &mut Entity);

    /**
     * 公共分页查询方法
     */
    async fn page(&self, arg: &Params, page: PageData) -> Result<Page<Dto>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        //构建查询条件
        let wrapper = Self::get_wrapper(arg).order_by(false, &vec!["id"]);
        //构建分页条件
        let page_request =
            PageRequest::new(page.page_no.unwrap_or(1), page.page_size.unwrap_or(10));

        //执行分页查询
        let data_page: Page<Entity> = rb.fetch_page_by_wrapper(wrapper, &page_request).await?;

        let vos = data_page
            .records
            .into_iter()
            .map(|e| {
                Dto::from(e.clone())
            })
            .collect::<Vec<Dto>>();

        Ok(Page::<Dto> {
            records: vos,
            total: data_page.total,
            pages: data_page.pages,
            page_no: data_page.page_no,
            page_size: data_page.page_size,
            search_count: data_page.search_count,
        })
    }

    async fn fetch_list_by_column(
        &self,
        column: &str,
        column_values: &Vec<String>,
    ) -> Result<Vec<Dto>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        //执行查询
        let list: Vec<Entity> = rb.fetch_list_by_column(column, column_values).await?;
        let result = list
            .into_iter()
            .map(|e| Dto::from(e.clone()))
            .collect::<Vec<Dto>>();
        Ok(result)
    }

    async fn fetch_count_by_wrapper(&self, arg: &Params) -> Result<u64> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let wrapper = Self::get_wrapper(arg);
        //执行查询
        let count = rb.fetch_count_by_wrapper::<Entity>(wrapper).await?;
        Ok(count)
    }

    async fn fetch_by_wrapper(&self, arg: &Params) -> Result<Dto> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let wrapper = Self::get_wrapper(arg);
        //执行查询
        let detail = rb.fetch_by_wrapper::<Entity>(wrapper).await?;
        let vo = Dto::from(detail);
        Ok(vo)
    }
    /**
     * 公共列表查询方法
     */
    async fn list(&self, arg: &Params) -> Result<Vec<Dto>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        //构建查询条件
        let wrapper = Self::get_wrapper(arg).order_by(false, &vec!["id"]);
        println!("wrapper:{:?}",wrapper);
        //执行查询
        let list: Vec<Entity> = rb.fetch_list_by_wrapper(wrapper).await?;
        let result = list
            .into_iter()
            .map(|e| Dto::from(e.clone()))
            .collect::<Vec<Dto>>();
        Ok(result)
    }

    /**
     * 根据id更新实体
     */
    async fn update_by_id(&self, id: String, mut data: &Entity)->Result<u64> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let wrapper = rb.new_wrapper().eq("id", id);
        let resut = rb.update_by_wrapper(
            &mut data,
            wrapper,
            &[Skip::Column("id"), Skip::Column("created_at")],
        )
        .await.unwrap();
        Ok(resut)
    }

    /**
     * 根据id更新某个字段
     */
    async fn update_batch_column(&self, column: &str, data: &Vec<Entity>)->Result<u64> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let result = rb.update_batch_by_column(column, data).await;
        Ok(result.unwrap())
    }
    /**
     * 根据id更新实体
     */
    async fn update_by_id_skips(&self, id: String, mut data: &Entity, skips:&[Skip]) {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let wrapper = rb.new_wrapper().eq("id", id);
        rb.update_by_wrapper(
            &mut data,
            wrapper,
            skips,
        )
        .await;
    }

    /**
     * 根据id查询条件查询单个值
     */
    async fn get(&self, id: String) -> Result<Dto> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let wrapper = rb.new_wrapper().eq("id", id);
        let detail: Entity = rb.fetch_by_wrapper(wrapper).await?;
        let vo = Dto::from(detail);
        return Ok(vo);
    }

    /**
     * 根据查询条件查询单个值
     */
    async fn search(&self, arg: &HashMap<String,String>) -> Result<Vec<Dto>> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let mut wrapper = rb.new_wrapper();
        arg.into_iter().map(|(k, v)| {
            wrapper = rb.new_wrapper().like(k, v);
        });
        let list: Vec<Entity> = rb.fetch_list_by_wrapper(wrapper).await?;
        let result = list
            .into_iter()
            .map(|e| Dto::from(e.clone()))
            .collect::<Vec<Dto>>();
        return Ok(result);
    }

    /**
     * 根据其他字段查询查询条件查询单个值
     */
    async fn get_by(&self, name:String ,value: String) -> Result<Dto> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let wrapper = rb.new_wrapper().eq(name.as_str(), value);
        let detail: Entity = rb.fetch_by_wrapper(wrapper).await?;
        let vo: Dto = Dto::from(detail);
        return Ok(vo);
    }

    /**
     * 保存实体
     */
    async fn save(&self, data: &mut Entity) -> Result<i64> {
        /*设置创建人*/
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let uid = if let Some(request_model) = get_local() {
            request_model.uid().clone()
        } else {
            0
        };
        /*设置公共字段*/
        self.set_save_common_fields(
            CommonField {
                id: Some(0),
                created_at: Some(DateTimeNative::now()),
                updated_at: Some(DateTimeNative::now()),
            },
            data,
        );
        let result = rb.save(data, &[Skip::Column("id")]).await?;
        return Ok(result.last_insert_id.unwrap());
    }
    /**
     * 批量保存实体
     */
    async fn save_batch(&self, mut list: &Vec<Entity>) -> Result<u64>{
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let result = rb.save_batch(&mut list, &[Skip::Column("id")]).await;
        return Ok(result.unwrap().rows_affected);
    }
    /**
     * 删除实体 逻辑删除
     */
    async fn del(&self, id: &String)-> Result<u64> {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        let result = rb.remove_by_column::<Entity, _>("id", id).await;
        return Ok(result.unwrap());
    }
    /**
     * 根据字段实体
     */
    async fn del_by_column(&self, column: &str, column_value: &str) {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.remove_by_column::<Entity, _>(column, column_value).await;
    }
    /**
     * 批量删除实体 逻辑删除
     */
    async fn del_batch(&self, ids: &Vec<u64>) {
        let rb = APPLICATION_CONTEXT.get::<Rbatis>();
        rb.remove_batch_by_column::<Entity, _>("id", ids).await;
    }
}