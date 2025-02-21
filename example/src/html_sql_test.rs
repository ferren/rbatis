#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;

    use rbatis::executor::RbatisExecutor;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;

    use crate::BizActivity;

    ///select page must have  '?:&PageRequest' arg and return 'Page<?>'
    #[html_sql(rb, "example/example.html")]
    async fn select_by_condition(rb: &mut RbatisExecutor<'_>, page_req: &PageRequest, name: &str) -> Page<BizActivity> { todo!() }


    // #[html_sql(self, "example/example.html")]
    // #[async_trait]
    // pub trait Example{
    //     #[html_sql(rb, "example/example.html")]
    //     async fn select_by_condition(&self, page_req: &PageRequest, name: &str) -> Page<BizActivity>;
    // }
    //
    // #[async_trait]
    // impl Example for RbatisExecutor<'_>{
    //     async fn select_by_condition(page_req: &PageRequest, name: &str) -> Page<BizActivity> {
    //         todo!()
    //     }
    // }


    #[tokio::test]
    pub async fn test_py_select_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = select_by_condition(&mut rb.as_executor(), &PageRequest::new(1, 10), "test")
            .await
            .unwrap();
        println!("{:?}", a);
    }
}
