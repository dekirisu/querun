#![allow(dead_code)]
use querun::*;

#[derive(QuerioRGInput,Intuple)]
struct TestInput {
    num: u32
}

#[derive(Strung,Intuple)]
struct TestVariable {
    id: u32
}

#[derive(QuerioRGOutputJson,Debug)]
struct TestOutput {
    cypher: u32,
    strung: u32,
    section: u32,
}

#[derive(Querio,QuerunRG)]
#[querio(I(TestInput),O(TestOutput),V(TestVariable),S("1002"),r#"
    CYPHER <Input>
    WITH    $num as cypher, 
            #id as strung, 
            <0> as section
    RETURN <Output>
"#)]
struct TestQuery;

#[tokio::main]
async fn main() {
    println!("{}",TestQuery::qrio((50,),(),(501,)));

    let client = redis::Client::open("redis://127.0.0.1:7778").unwrap();
    let mut con = client.get_tokio_connection_manager().await.unwrap();
    //
    let a = TestQuery::qrun_async_json(&mut con,(12,),(),(46,)).await.unwrap();
    println!("{}",a);
}