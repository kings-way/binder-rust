extern crate binder_rust;
use binder_rust::ServiceManager;

fn main() {
    let service_name = "tethering";
    println!("==== get_service(): ====\nservice: [{}]", service_name);
    let service_manager = &mut ServiceManager::new().unwrap();
    let _svc = service_manager.get_service(service_name, "Nothing").unwrap();

    println!("\n==== list_service() ====");
    let service_manager = &mut ServiceManager::new().unwrap();
    let _svc = service_manager.list_service().unwrap();

}

