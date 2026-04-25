use macroquad::prelude::*;
use macroquad::rand::*;

#[macroquad::main("Chess Tools")]
async fn main() {
    let size = screen_width() / 3.0;
    let poses: Vec<(f32, f32)> = vec![(0.0, 0.0), (size, 0.0), (2.0 * size, 0.0)];
    let textures: Vec<Texture2D> = get_textures().await;
    let mut chosen_textures: Vec<&Texture2D> = vec![&textures[0], &textures[1], &textures[2]];

    let draw_params = DrawTextureParams{
        dest_size: Some(vec2(size, size)), 
        ..Default::default()
    };
    
    loop {
        clear_background(WHITE);

        if is_mouse_button_down(MouseButton::Left) || !touches().is_empty(){
            for x in 0..chosen_textures.len(){
                chosen_textures[x] = &textures[gen_range(0, textures.len())];
            }
        }
        
        for x in 0..poses.len(){
            draw_texture_ex(chosen_textures[x], poses[x].0, poses[x].1, WHITE, draw_params.clone());
        }

        next_frame().await
    }
}

async fn get_textures() -> Vec<Texture2D>{
    let mut textures: Vec<Texture2D> = vec![];
    textures.push(load_texture("assets/chess_pieces/Chess_plt45.png").await.expect("failed to load texture"));
    textures.push(load_texture("assets/chess_pieces/Chess_nlt45.png").await.expect("failed to load texture"));
    textures.push(load_texture("assets/chess_pieces/Chess_blt45.png").await.expect("failed to load texture"));
    textures.push(load_texture("assets/chess_pieces/Chess_rlt45.png").await.expect("failed to load texture"));
    textures.push(load_texture("assets/chess_pieces/Chess_qlt45.png").await.expect("failed to load texture"));
    textures.push(load_texture("assets/chess_pieces/Chess_klt45.png").await.expect("failed to load texture"));
    textures
}