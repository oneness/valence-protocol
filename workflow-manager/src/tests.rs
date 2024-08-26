#[cfg(test)]
mod test {
    use std::collections::{BTreeMap, BTreeSet};

    use cosmwasm_std::Uint128;
    use serde_json_any_key::MapIterToJson;
    use services_utils::ServiceAccountType;
    use valence_splitter::msg::ServiceConfig as SplitterServiceConfig;

    use crate::{
        account::{AccountInfo, AccountType},
        domain::Domain,
        init_workflow,
        service::{ServiceConfig, ServiceInfo},
        workflow_config::{Link, WorkflowConfig},
    };

    #[global_allocator]
    static ALLOC: dhat::Alloc = dhat::Alloc;

    #[tokio::test]
    async fn test_domains() {
        // let _profiler = dhat::Profiler::builder().testing().build();

        // let domain = Domain::Cosmos("cosmos".to_string());
        // // let domain2 = Domain::Cosmos("neutron".to_string());
        // let mut domain_info = DomainInfo::from_domain(&domain).await;
        // println!("{domain_info:?}");
        // let mut domain_info2 = DomainInfo::from_domain(domain2).await;
        // println!("{domain_info2:?}");

        // let mut domain_info3 = DomainInfo::from_domain(domain).await;
        // println!("{domain_info3:?}");

        // let stats = dhat::HeapStats::get();

        // let d = domain_info
        //     .connector
        //     .init_account(
        //         1,
        //         None,
        //         "label".to_string(),
        //     )
        //     .await;
        // println!("Balance: {d:?}");
    }

    #[test]
    fn test_serialize() {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
        struct Test {
            test: ServiceAccountType,
        }

        let t = Test {
            test: ServiceAccountType::AccountId(1),
        };

        // let mut json = serde_json::to_string(&t).unwrap();
        let mut json = serde_json::to_string(&t).unwrap();
        println!("{json:?}");

        json = json.replace("|account_id|\":1", "account_addr\":\"cosmos1\"");
        println!("{json:?}");

        let back_struct = serde_json::from_str::<Test>(&json).unwrap();
        println!("{back_struct:?}");

        let mut splits: BTreeMap<ServiceAccountType, Uint128> = BTreeMap::new();
        splits.insert(ServiceAccountType::AccountId(2), 100_u128.into());
        splits.insert(ServiceAccountType::AccountId(3), 200_u128.into());
        // let to_vec = splits;
        let json = splits.to_json_map().unwrap();
        // let json = serde_json::to_string(&splits).unwrap();
        println!("{json:?}");
    }

    #[tokio::test]
    async fn test() {
        // let subscriber = tracing_subscriber::fmt()
        //     .with_max_level(tracing::Level::DEBUG)
        //     .with_test_writer()
        //     .with_span_events(FmtSpan::CLOSE)
        //     .finish();
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("setting default subscriber failed");

        let mut config = WorkflowConfig::default();

        config.accounts.insert(
            1,
            AccountInfo {
                ty: AccountType::Base { admin: None },
                domain: Domain::CosmosCosmwasm("neutron".to_string()),
            },
        );
        config.accounts.insert(
            2,
            AccountInfo {
                ty: AccountType::Base { admin: None },
                domain: Domain::CosmosCosmwasm("neutron".to_string()),
            },
        );
        config.accounts.insert(
            3,
            AccountInfo {
                ty: AccountType::Base { admin: None },
                domain: Domain::CosmosCosmwasm("neutron".to_string()),
            },
        );

        let mut splits: BTreeSet<(ServiceAccountType, Uint128)> = BTreeSet::new();
        splits.insert((ServiceAccountType::AccountId(2), 100_u128.into()));
        splits.insert((ServiceAccountType::AccountId(3), 200_u128.into()));

        config.services.insert(
            1,
            ServiceInfo {
                domain: Domain::CosmosCosmwasm("neutron".to_string()),
                config: ServiceConfig::Splitter(SplitterServiceConfig {
                    input_addr: ServiceAccountType::AccountId(1),
                    splits: (BTreeMap::from_iter(vec![("NTRN".to_string(), splits)].into_iter())),
                }),
            },
        );

        config.links.insert(
            1,
            Link {
                input_accounts_id: vec![1],
                output_accounts_id: vec![2, 3],
                service_id: 1,
            },
        );

        init_workflow(config).await;

        // match timeout(Duration::from_secs(60), ).await {
        //     Ok(_) => println!("Workflow initialization completed successfully"),
        //     Err(_) => println!("Workflow initialization timed out after 60 seconds"),
        // }
    }
}