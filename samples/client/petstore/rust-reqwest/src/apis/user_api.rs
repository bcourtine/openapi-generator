/* 
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::collections::HashMap;

use reqwest;

use super::{Error, configuration};

pub struct UserApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl UserApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> UserApiClient {
        UserApiClient {
            configuration: configuration,
        }
    }
}

pub trait UserApi {
    fn create_user(&self, user: ::models::User) -> Result<(), Error>;
    fn create_users_with_array_input(&self, user: Vec<::models::User>) -> Result<(), Error>;
    fn create_users_with_list_input(&self, user: Vec<::models::User>) -> Result<(), Error>;
    fn delete_user(&self, username: &str) -> Result<(), Error>;
    fn get_user_by_name(&self, username: &str) -> Result<::models::User, Error>;
    fn login_user(&self, username: &str, password: &str) -> Result<String, Error>;
    fn logout_user(&self, ) -> Result<(), Error>;
    fn update_user(&self, username: &str, user: ::models::User) -> Result<(), Error>;
}


impl UserApi for UserApiClient {
    fn create_user(&self, user: ::models::User) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/user?{}", configuration.base_path, query_string);

        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }



        req_builder = req_builder.json(&user);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn create_users_with_array_input(&self, user: Vec<::models::User>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/user/createWithArray?{}", configuration.base_path, query_string);

        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }



        req_builder = req_builder.json(&user);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn create_users_with_list_input(&self, user: Vec<::models::User>) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/user/createWithList?{}", configuration.base_path, query_string);

        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }



        req_builder = req_builder.json(&user);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn delete_user(&self, username: &str) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/user/{username}?{}", configuration.base_path, query_string, username=username);

        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }




        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn get_user_by_name(&self, username: &str) -> Result<::models::User, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/user/{username}?{}", configuration.base_path, query_string, username=username);

        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }




        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn login_user(&self, username: &str, password: &str) -> Result<String, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("username", &username.to_string());
            query.append_pair("password", &password.to_string());

            query.finish()
        };
        let uri_str = format!("{}/user/login?{}", configuration.base_path, query_string);

        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }




        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn logout_user(&self, ) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/user/logout?{}", configuration.base_path, query_string);

        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }




        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

    fn update_user(&self, username: &str, user: ::models::User) -> Result<(), Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());

            query.finish()
        };
        let uri_str = format!("{}/user/{username}?{}", configuration.base_path, query_string, username=username);

        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }



        req_builder = req_builder.json(&user);

        // send request
        let req = req_builder.build()?;

        client.execute(req)?.error_for_status()?;
        Ok(())
    }

}
