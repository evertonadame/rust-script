mod utils;
mod structs;
use reqwest::header::{HeaderMap, HeaderValue};
use log::{error, info, warn};
use std::fs;
use dotenv::dotenv;

fn main() {
    dotenv().ok();   

    env_logger::init();                                                                                                                                                                                                                                             

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        if let Err(e) = get_all_templates().await {
            error!("Error: {}", e);
        }
    });

}

async fn get_all_templates() -> Result<(), reqwest::Error> {
    let mut time = utils::ExecutionTimer::new();

    let codegen_start = std::time::Instant::now();

    let get_all_templates_path = "/component-templates?pageSize=100";
    let formatted_url = utils::format_url_path(&get_all_templates_path);

    let client = reqwest::Client::new();

    let headers = get_req_common_headers();

    let api_start = std::time::Instant::now();
    let res = client.get(&formatted_url)
        .headers(headers).send().await?;
    time.add_timing("api_request".to_string(), api_start.elapsed());

    if res.status().is_success() {
        let res = res.json::<structs::TemplatesResponse>().await?;

        let templates_from_folder = get_templates_folder();
        let templates_from_api = res.data.templates;

        check_templates_diffs(&templates_from_folder, &templates_from_api);

        for mut template_api in templates_from_api {
            let template_api_concat_name = format!("{}-{}", utils::format_maybe_string(&template_api.component_type), utils::format_maybe_string(&template_api.render_type));
            
            let template_from_folder = templates_from_folder.iter().find(|template| template.name == template_api_concat_name);

            if let Some(template) = template_from_folder {

                template_api.schema = structs::Schema {
                        json: Some(template.content.json.clone().unwrap_or_else(utils::empty_json)),
                        ui: Some(template.content.ui.clone().unwrap_or_else(utils::empty_json)),
                    };

                    let api_start = std::time::Instant::now();

                    // put all templates in the API
                    // put_template_in_api(template_api).await?;

                    // test for one template
                    if utils::format_maybe_string(&template_api.render_type).contains("box") {
                        put_template_in_api(template_api).await?;
                    }
                        
                    time.add_timing("api_request".to_string(), api_start.elapsed());


            } else {
                warn!("There is no template named {} in the final templates folder.", template_api_concat_name);
            }

        }
    } else {
        let error = res.bytes().await?;
        error!("Error on get templates: {:?}", error);
    }

    time.add_timing("code_execution".to_string(), codegen_start.elapsed());

    time.log_timings();

    Ok(())
}

async fn put_template_in_api(template: structs::Template) -> Result<(), reqwest::Error>{

    let put_template_path = format!("/component-templates/{}", utils::format_maybe_string(&template.id));

    let formatted_url = utils::format_url_path(&put_template_path);

    let client = reqwest::Client::new();

    let headers = get_req_common_headers();
    
    let template_without_id: structs::TemplateWithoutId = template.into();

    let res = client.put(&formatted_url)
        .headers(headers)
        .json(&template_without_id)
        .send().await?;

    if res.status().is_success() {
        info!("Template {} updated successfully", utils::format_maybe_string(&template_without_id.name));
    } else {
        let error = res.bytes().await?;
        error!("Error on updating template: {:?}, name:{:?}", error, &template_without_id.name);
    }

    Ok(())
}

fn get_templates_folder() -> Vec<structs::TemplateFile> {
    let directory_path = "src/final-templates";
    let mut templates = Vec::new();

    match fs::read_dir(directory_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let file_path = entry.path();
                if let Some(file_name) = file_path.file_name().and_then(|s| s.to_str()) {
                    if file_name.ends_with(".json") {
                        if let Ok(file_content) = fs::read_to_string(&file_path) {
                            if let Ok(json_content) = serde_json::from_str::<structs::Schema>(&file_content) {
                                templates.push(structs::TemplateFile {
                                    name: file_name.trim_end_matches(".json").to_string(),
                                    content: structs::Schema { json: json_content.json, ui: json_content.ui},
                                });
                            } else {
                                error!("Erro ao converter o conteúdo do arquivo {} em JSON", file_name);
                            }
                        } else {
                            error!("Erro ao ler o arquivo {}", file_name);
                        }
                    }
                }
            }
        }
        Err(err) => {
            error!("Erro ao acessar a pasta {}: {}", directory_path, err);
        }
    }

    templates
}

fn check_templates_diffs(templates_from_folder: &Vec<structs::TemplateFile>, templates_from_api: &Vec<structs::Template>) {
    let channel_id = std::env::var("CHANNEL_ID").expect("CHANNEL_ID must be set");
    
    let templates_from_folder_len = templates_from_folder.len();
        let templates_from_api_len = templates_from_api.len();

        info!("The number of final templates in the project folder: length: {} templates", templates_from_folder_len);
        info!("Quantity of templates in this channel {}: length: {} templates", channel_id, templates_from_api_len);

        if templates_from_folder_len < templates_from_api_len {
            warn!("The quantity of templates in the final templates folder is less than the quantity of templates in the API, there might be duplicated or missing templates in the final templates folder.");
        } else if templates_from_folder_len > templates_from_api_len {
            info!("The quantity of templates in the final templates folder is greater than the quantity of templates in the API; your website may have fewer");
        } else {
            info!("The quantity of templates in the final templates folder is equal to the quantity of templates in the API.");
        }
}

fn get_req_common_headers() -> HeaderMap {
    let tenant_id = std::env::var("TENANT_ID").expect("TENANT_ID must be set");
    let channel_id = std::env::var("CHANNEL_ID").expect("CHANNEL_ID must be set");
    let token = std::env::var("TOKEN").expect("TOKEN must be set");

    let formatted_token = format!("Bearer {}", token);
    
    let mut headers = HeaderMap::new();

    headers.insert("Authorization", HeaderValue::from_str(&formatted_token).unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    headers.insert("X-Channel-Id", HeaderValue::from_str(&channel_id).unwrap());
    headers.insert("X-Tenant-Id", HeaderValue::from_str(&tenant_id).unwrap());

    headers
}
