use gstd::{msg, prelude::*, ActorId, CodeId};

async fn create_this(code_hash: &CodeId) -> ActorId {
    let (_, actor_id) =
        gstd::prog::ProgramGenerator::create_program_bytes(code_hash.clone(), b"PING", 0)
            .expect("Failed to create this/self");

    actor_id
}

#[gear_test_codegen::test]
async fn good(context: &gear_test_runtime::SessionData) {
    let this = create_this(&context.testee()).await;

    let result: Vec<u8> = msg::send_bytes_for_reply(this, b"PING", 0, 0)
        .expect("failed to send")
        .await
        .expect("Program to handle simple PING!!1");

    assert_eq!(result, b"PONG")
}

#[gear_test_codegen::test]
async fn bad(context: &gear_test_runtime::SessionData) {
    let this = create_this(&context.testee()).await;

    let result: Vec<u8> = msg::send_bytes_for_reply(this, b"PING", 0, 0)
        .expect("failed to send")
        .await
        .expect("Program to handle simple PING!!1");

    assert_eq!(result, b"NOTPOING")
}
