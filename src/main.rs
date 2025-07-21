use calc_near_x::calc_with_1;
use tide::Request;


#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("☀ Starting server...");
    let mut app = tide::new();
    app.at("/").get(|_| async { Ok("😜 Hello Tide!") });
    app.at("/plus_one/:x/:y").get(plus_one);
    app.at("/less_one/:x/:y").get(less_one);
    println!("😎 Running - Listening on port 3000");
    app.listen("0.0.0.0:3000").await?;

    Ok(())
}

async fn plus_one(req: Request<()>) -> tide::Result {
    let x = req.param("x").unwrap_or("0").parse().unwrap_or(0);
    let y = req.param("y").unwrap_or("0").parse().unwrap_or(0);

    let z = calc_with_1::sum_plus_one(x,y);

    Ok(format!("Resultado: {}", z).into())
}

async fn less_one(req: Request<()>) -> tide::Result {
    let x = req.param("x").unwrap_or("0").parse().unwrap_or(0);
    let y = req.param("y").unwrap_or("0").parse().unwrap_or(0);

    let z = calc_with_1::sub_less_one(x,y);

    Ok(format!("Resultado: {}", z).into())
}