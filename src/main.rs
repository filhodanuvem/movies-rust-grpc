use std::net::TcpListener;
use std::env;
use tonic::{transport::Server, Request, Response, Status};

pub mod grpc_movie {
    tonic::include_proto!("movie");
}
use grpc_movie::movie_server::{Movie, MovieServer};
use grpc_movie::{MovieRequest, MovieResponse};

#[derive(Debug, Default)]
pub struct MovieService {}

#[tonic::async_trait]
impl Movie for MovieService {
    async fn get_movies(
        &self,
        request: Request<MovieRequest>,
    ) -> Result<Response<MovieResponse>, Status> {
        println!("Got a request: {:?}", request);

        let mut movies = Vec::new();
        movies.push(grpc_movie::MovieItem {
            id: 1,
            title: "Matrix".to_string(),
            year: 1999,
            genre: "Sci-Fi".to_string(),
            rating: "8.7".to_string(),
            start_rating: "4.5".to_string(),
            runtime: "136".to_string(),
            cast: "Keanu Reeves, Laurence Fishburne".to_string(),
            image: "http://image.tmdb.org/t/p/w500//aOIuZAjPaRIE6CMzbazvcHuHXDc.jpg".to_string(),
        });
        movies.push(grpc_movie::MovieItem {
            id: 2,
            title: "Spider-Man: Across the Spider-Verse".to_string(),
            year: 2023,
            genre: "Animation".to_string(),
            rating: "9.7".to_string(),
            start_rating: "4.5".to_string(),
            runtime: "136".to_string(),
            cast: "Donald Glover".to_string(),
            image: "http://image.tmdb.org/t/p/w500//8Vt6mWEReuy4Of61Lnj5Xj704m8.jpg".to_string(),
        });
        movies.push(grpc_movie::MovieItem {
            id: 3,
            title: "Her".to_string(),
            year: 2013,
            genre: "Drama".to_string(),
            rating: "8.7".to_string(),
            start_rating: "4.5".to_string(),
            runtime: "136".to_string(),
            cast: "Joaquin Phoenix".to_string(),
            image: "http://image.tmdb.org/t/p/w500//eCOtqtfvn7mxGl6nfmq4b1exJRc.jpg".to_string(),
        });

        let reply = grpc_movie::MovieResponse { movies: movies };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let front_url = env::var("FRONTEND_URL").unwrap_or("localhost:3000".to_string());
    let port = env::var("PORT").unwrap_or("50051".to_string());
    println!("FRONTEND_URL: {}", front_url);
    let addr = format!("127.0.0.1:{}", port).parse()?;
    let movie = MovieService::default();
    let movie = MovieServer::new(movie);
    let movie = tonic_web::config().allow_all_origins().enable(movie);
    let tcp_listener = TcpListener::bind("127.0.0.1:50052").unwrap();

    Server::builder()
        .accept_http1(true)
        .add_service(movie)
        .serve(addr)
        .await?;

    drop(tcp_listener);
    Ok(())
}
