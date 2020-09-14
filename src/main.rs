use actix_web::{
    body::{BodySize, MessageBody, ResponseBody},
    Error,
};
use bytes::Bytes;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct StreamLog<B: MessageBody> {
    body: ResponseBody<B>,
    resp_status: u16,
}

impl<B: MessageBody> MessageBody for StreamLog<B> {
    fn size(&self) -> BodySize {
        self.body.size()
    }

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Bytes, Error>>> {
        let body: ResponseBody<B> = self.body;

        // The next line gives a compile error:
        // error[E0599]: no method named `poll_next` found for enum
        // `actix_web::dev::ResponseBody<B>` in the current scope
        //
        // This is confusing because I see in the docs that ResponseBody<B>
        // does impl MessageBody provided B: MessageBody. And B is constrained
        // to impl it. And poll_next is a required method of MessageBody!
        // https://docs.rs/actix-web/3.0.1/actix_web/dev/trait.MessageBody.html#impl-MessageBody-3
        match body.poll_next(cx) {
            Poll::Ready(Some(Ok(chunk))) => Poll::Ready(Some(Ok(chunk))),
            val => val,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
