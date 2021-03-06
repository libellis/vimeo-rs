//!
//! This example showcases the Vimeo OAuth2 process for requesting access to the user's private assets.
//!
//! Before running it, you'll need to generate your own Vimeo OAuth2 credentials.
//!
//! In order to run the example call:
//!
//! ```sh
//! VIMEO_CLIENT_ID=xxx VIMEO_CLIENT_SECRET=yyy cargo run --example vimeo-oauth
//! ```
//!
//! ...and follow the instructions.
//!

use oauth2::basic::BasicClient;

// Alternatively, this can be `oauth2::curl::http_client` or a custom client.
use oauth2::reqwest::http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use tokio::runtime::Runtime;
use url::Url;

use vimeo_rs::{Client, UserService};

fn main() {
    let vimeo_client_id = ClientId::new(
        env::var("VIMEO_CLIENT_ID").expect("Missing the VIMEO_CLIENT_ID environment variable."),
    );

    let vimeo_client_secret = ClientSecret::new(
        env::var("VIMEO_CLIENT_SECRET")
            .expect("Missing the VIMEO_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new("https://api.vimeo.com/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://api.vimeo.com/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");

    // Set up the config for the Vimeo OAuth2 process.
    let client = BasicClient::new(
        vimeo_client_id,
        Some(vimeo_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_url(
        RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
    );

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's public repos and email.
        .add_scope(Scope::new("private".to_string()))
        .url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );

    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let code;
            let state: CsrfToken;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            println!("Vimeo returned the following code:\n{}\n", code.secret());
            println!(
                "Vimeo returned the following state:\n{} (expected `{}`)\n",
                state.secret(),
                csrf_state.secret()
            );

            // TODO: Error out here if states don't match.

            // Exchange the code with a token.
            let token_res = client.exchange_code(code).request(http_client);

            println!(
                "Vimeo returned the following token response:\n{:?}\n",
                token_res
            );

            if let Ok(mut token) = token_res {
                let scopes = if let Some(scopes_vec) = token.scopes() {
                    scopes_vec
                        .iter()
                        .map(|comma_separated| comma_separated.split(','))
                        .flatten()
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                };

                println!("Vimeo returned the following scopes:\n{:?}\n", scopes);
                println!(
                    "Vimeo returned the following token:\n{:?}\n",
                    token.access_token().secret()
                );
                env::set_var("VIMEO_ACCESS_TOKEN", token.access_token().secret())
            }

            // The server will terminate itself after collecting the first code.
            break;
        }
    }

    let access_token = env::var("VIMEO_ACCESS_TOKEN").unwrap();

    let vimeo = Client::new(access_token);

    // spawn a tokio runtime - easy for testing purposes to do it manually here.
    let mut rt = Runtime::new().unwrap();

    let user = rt.block_on(vimeo.users().get(None)).unwrap();
    println!("User from /me endpoint:\n{:?}\n", user);

    let videos = rt.block_on(vimeo.videos().list(None)).unwrap();
    println!("Videos from /me endpoint:\n{:?}\n", videos);
}
