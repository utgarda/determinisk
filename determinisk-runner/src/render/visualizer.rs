//! Macroquad-based visualizer for simulation traces

use determinisk_core::{SimulationTrace, CircleState};
use macroquad::prelude::*;
use serde::{Serialize, Deserialize};

const PIXELS_PER_METER: f32 = 50.0;

/// Proof metrics from zkVM execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofMetrics {
    pub total_cycles: u64,
    pub user_cycles: Option<u64>,
    pub segments: u32,
    pub proof_size_bytes: usize,
    pub proving_time_ms: u128,
    pub verification_time_ms: Option<u128>,
    pub zkvm_backend: String,
}

pub struct Visualizer {
    trace: SimulationTrace,
    current_frame: usize,
    playing: bool,
    _playback_speed: f32,
    show_trails: bool,
    show_velocities: bool,
    show_metrics: bool,
    show_grid: bool,
    trail_length: usize,
    proof_metrics: Option<ProofMetrics>,
}

impl Visualizer {
    pub fn new(trace: SimulationTrace) -> Self {
        Self {
            trace,
            current_frame: 0,
            playing: true,
            _playback_speed: 1.0,
            show_trails: true,
            show_velocities: true,
            show_metrics: true,
            show_grid: true,
            trail_length: 30,
            proof_metrics: None,
        }
    }
    
    pub fn with_proof_metrics(trace: SimulationTrace, proof_metrics: ProofMetrics) -> Self {
        Self {
            trace,
            current_frame: 0,
            playing: true,
            _playback_speed: 1.0,
            show_trails: true,
            show_velocities: true,
            show_metrics: true,
            show_grid: true,
            trail_length: 30,
            proof_metrics: Some(proof_metrics),
        }
    }
    
    fn handle_input(&mut self) {
        // Check for input
        if is_key_pressed(KeyCode::Space) {
            self.playing = !self.playing;
        }
        
        if is_key_pressed(KeyCode::Left) && self.current_frame > 0 {
            self.current_frame -= 1;
            self.playing = false;
        }
        
        if is_key_pressed(KeyCode::Right) && self.current_frame < self.trace.states.len() - 1 {
            self.current_frame += 1;
            self.playing = false;
        }
        
        if is_key_pressed(KeyCode::R) {
            self.current_frame = 0;
        }
        
        if is_key_pressed(KeyCode::T) {
            self.show_trails = !self.show_trails;
        }
        
        if is_key_pressed(KeyCode::V) {
            self.show_velocities = !self.show_velocities;
        }
        
        if is_key_pressed(KeyCode::G) {
            self.show_grid = !self.show_grid;
        }
        
        if is_key_pressed(KeyCode::M) {
            self.show_metrics = !self.show_metrics;
        }
    }
    
    fn world_to_screen(&self, pos: [f32; 2]) -> (f32, f32) {
        let x = pos[0] * PIXELS_PER_METER;
        let y = (self.trace.input.world_height - pos[1]) * PIXELS_PER_METER;
        (x, y)
    }
    
    fn draw_grid(&self) {
        let grid_color = Color::new(0.3, 0.3, 0.3, 0.3);
        let width = self.trace.input.world_width;
        let height = self.trace.input.world_height;
        
        // Vertical lines
        for i in 0..=(width as i32) {
            let x = i as f32 * PIXELS_PER_METER;
            draw_line(x, 0.0, x, height * PIXELS_PER_METER, 1.0, grid_color);
        }
        
        // Horizontal lines
        for i in 0..=(height as i32) {
            let y = i as f32 * PIXELS_PER_METER;
            draw_line(0.0, y, width * PIXELS_PER_METER, y, 1.0, grid_color);
        }
    }
    
    fn draw_boundaries(&self) {
        let width = self.trace.input.world_width * PIXELS_PER_METER;
        let height = self.trace.input.world_height * PIXELS_PER_METER;
        let color = RED;
        let thickness = 3.0;
        
        draw_line(0.0, 0.0, width, 0.0, thickness, color);
        draw_line(0.0, height, width, height, thickness, color);
        draw_line(0.0, 0.0, 0.0, height, thickness, color);
        draw_line(width, 0.0, width, height, thickness, color);
    }
    
    fn draw_circle(&self, circle: &CircleState, color: Color) {
        let (x, y) = self.world_to_screen(circle.position);
        let radius = circle.radius * PIXELS_PER_METER;
        
        draw_circle(x, y, radius, color);
        draw_circle_lines(x, y, radius, 2.0, WHITE);
    }
    
    fn draw_velocity(&self, circle: &CircleState) {
        let (x, y) = self.world_to_screen(circle.position);
        let scale = 20.0;
        let vx = circle.velocity[0] * scale;
        let vy = -circle.velocity[1] * scale;
        
        if vx.abs() > 0.1 || vy.abs() > 0.1 {
            draw_line(x, y, x + vx, y + vy, 2.0, GREEN);
            
            // Arrowhead
            let angle = vy.atan2(vx);
            let arrow_size = 5.0;
            draw_triangle(
                vec2(x + vx, y + vy),
                vec2(
                    x + vx - arrow_size * (angle + 2.5).cos(),
                    y + vy - arrow_size * (angle + 2.5).sin(),
                ),
                vec2(
                    x + vx - arrow_size * (angle - 2.5).cos(),
                    y + vy - arrow_size * (angle - 2.5).sin(),
                ),
                GREEN,
            );
        }
    }
    
    fn draw_trails(&self) {
        let start = self.current_frame.saturating_sub(self.trail_length);
        let end = self.current_frame;
        
        for (circle_idx, _circle) in self.trace.states[self.current_frame].circles.iter().enumerate() {
            let mut trail_points = Vec::new();
            
            for frame in start..=end {
                if frame < self.trace.states.len() {
                    let pos = self.trace.states[frame].circles[circle_idx].position;
                    let (x, y) = self.world_to_screen(pos);
                    trail_points.push(vec2(x, y));
                }
            }
            
            // Draw trail as fading line segments
            for i in 1..trail_points.len() {
                let alpha = (i as f32) / (trail_points.len() as f32);
                let color = Color::new(0.5, 0.7, 1.0, alpha * 0.5);
                draw_line(
                    trail_points[i-1].x,
                    trail_points[i-1].y,
                    trail_points[i].x,
                    trail_points[i].y,
                    2.0,
                    color,
                );
            }
        }
    }
    
    fn draw_ui(&self) {
        let state = &self.trace.states[self.current_frame];
        let metrics = &self.trace.output.metrics;
        
        // Input parameters
        draw_text("INPUT PARAMETERS", 10.0, 25.0, 24.0, YELLOW);
        draw_text(&format!("World: {}x{} m", 
            self.trace.input.world_width, 
            self.trace.input.world_height), 10.0, 50.0, 20.0, WHITE);
        draw_text(&format!("Gravity: ({:.1}, {:.1}) m/s²", 
            self.trace.input.gravity[0], 
            self.trace.input.gravity[1]), 10.0, 75.0, 20.0, WHITE);
        draw_text(&format!("Timestep: {:.4} s", 
            self.trace.input.timestep), 10.0, 100.0, 20.0, WHITE);
        draw_text(&format!("Circles: {}", 
            self.trace.input.circles.len()), 10.0, 125.0, 20.0, WHITE);
        
        // Current state
        draw_text("SIMULATION STATE", 10.0, 165.0, 24.0, YELLOW);
        draw_text(&format!("Frame: {}/{}", 
            self.current_frame, 
            self.trace.states.len() - 1), 10.0, 190.0, 20.0, WHITE);
        draw_text(&format!("Time: {:.2} s", state.time), 10.0, 215.0, 20.0, WHITE);
        draw_text(&format!("Step: {}", state.step), 10.0, 240.0, 20.0, WHITE);
        
        // Proof Data Metrics - moved lower
        let proof_y = 280.0;
        if let Some(proof) = &self.proof_metrics {
            // Actual proof metrics from zkVM
            draw_text("PROOF METRICS (Actual)", 10.0, proof_y, 24.0, GREEN);
            draw_text(&format!("Backend: {}", proof.zkvm_backend), 10.0, proof_y + 25.0, 20.0, WHITE);
            draw_text(&format!("Total cycles: {}", proof.total_cycles), 10.0, proof_y + 50.0, 20.0, WHITE);
            if let Some(user_cycles) = proof.user_cycles {
                draw_text(&format!("User cycles: {}", user_cycles), 10.0, proof_y + 75.0, 20.0, WHITE);
            }
            draw_text(&format!("Segments: {}", proof.segments), 10.0, proof_y + 100.0, 20.0, WHITE);
            draw_text(&format!("Proof size: {:.1} KB", 
                proof.proof_size_bytes as f32 / 1024.0), 10.0, proof_y + 125.0, 20.0, GREEN);
            draw_text(&format!("Proving time: {:.2} s", 
                proof.proving_time_ms as f32 / 1000.0), 10.0, proof_y + 150.0, 20.0, WHITE);
            if let Some(verify_time) = proof.verification_time_ms {
                draw_text(&format!("Verify time: {:.3} s", 
                    verify_time as f32 / 1000.0), 10.0, proof_y + 175.0, 20.0, WHITE);
            }
            
            // Event counts
            draw_text("EVENTS", 10.0, proof_y + 210.0, 24.0, Color::new(0.0, 1.0, 1.0, 1.0));
            draw_text(&format!("Total collisions: {}", 
                metrics.collision_count), 10.0, proof_y + 235.0, 20.0, WHITE);
            draw_text(&format!("Boundary events: {}", 
                metrics.boundary_hits), 10.0, proof_y + 260.0, 20.0, WHITE);
        } else {
            // Estimated metrics (no proof available)
            draw_text("PROOF DATA (Estimated)", 10.0, proof_y, 24.0, Color::new(0.0, 1.0, 1.0, 1.0));
            let num_circles = self.trace.input.circles.len();
            let num_frames = self.trace.states.len();
            let data_per_circle = 4 * 4; // position (2 * f32) + velocity (2 * f32)
            let data_per_frame = num_circles * data_per_circle;
            let total_data = num_frames * data_per_frame;
            
            draw_text(&format!("Bodies: {} × {} frames", 
                num_circles, num_frames), 10.0, proof_y + 25.0, 20.0, WHITE);
            draw_text(&format!("Data/frame: {} bytes", 
                data_per_frame), 10.0, proof_y + 50.0, 20.0, WHITE);
            draw_text(&format!("Total positions: {}", 
                num_circles * num_frames), 10.0, proof_y + 75.0, 20.0, WHITE);
            draw_text(&format!("Trajectory size: {:.1} KB", 
                total_data as f32 / 1024.0), 10.0, proof_y + 100.0, 20.0, WHITE);
            
            // Event counts
            draw_text("EVENTS", 10.0, proof_y + 135.0, 24.0, Color::new(0.0, 1.0, 1.0, 1.0));
            draw_text(&format!("Total collisions: {}", 
                metrics.collision_count), 10.0, proof_y + 160.0, 20.0, WHITE);
            draw_text(&format!("Avg per frame: {:.1}", 
                metrics.collision_count as f32 / num_frames as f32), 10.0, proof_y + 185.0, 20.0, WHITE);
            draw_text(&format!("Boundary events: {}", 
                metrics.boundary_hits), 10.0, proof_y + 210.0, 20.0, WHITE);
            
            // Estimated proof size
            let proof_overhead = 1024; // Base proof overhead
            let state_hash_size = 32; // SHA-256 hash
            let public_outputs = num_circles * 8 + 4; // Final positions + step count
            let estimated_proof_size = proof_overhead + public_outputs + state_hash_size;
            
            draw_text(&format!("Est. proof size: {:.1} KB", 
                estimated_proof_size as f32 / 1024.0), 10.0, proof_y + 245.0, 20.0, YELLOW);
        }
        
        // Metrics
        if self.show_metrics {
            let x = screen_width() - 250.0;
            draw_text("PHYSICS METRICS", x, 25.0, 24.0, YELLOW);
            draw_text(&format!("Total Energy: {:.2} J", 
                metrics.total_energy), x, 50.0, 20.0, WHITE);
            draw_text(&format!("Max Velocity: {:.2} m/s", 
                metrics.max_velocity), x, 75.0, 20.0, WHITE);
            
            // Computation complexity
            draw_text("COMPUTATION", x, 100.0, 18.0, YELLOW);
            let num_circles = self.trace.input.circles.len();
            let num_frames = self.trace.states.len();
            let collision_checks = (num_circles * (num_circles - 1)) / 2;
            draw_text(&format!("Collision pairs/frame: {}", 
                collision_checks), x, 120.0, 16.0, WHITE);
            draw_text(&format!("Total checks: {}", 
                collision_checks * num_frames), x, 140.0, 16.0, WHITE);
            
            // Current frame stats
            draw_text("CURRENT FRAME", x, 180.0, 18.0, YELLOW);
            let mut moving_count = 0;
            let mut max_speed = 0.0f32;
            for circle in &state.circles {
                let speed = (circle.velocity[0].powi(2) + 
                            circle.velocity[1].powi(2)).sqrt();
                if speed > 0.1 {
                    moving_count += 1;
                }
                max_speed = max_speed.max(speed);
            }
            draw_text(&format!("Bodies moving: {}/{}", 
                moving_count, state.circles.len()), x, 200.0, 16.0, WHITE);
            draw_text(&format!("Max speed: {:.2} m/s", 
                max_speed), x, 220.0, 16.0, WHITE);
            
            // Live collision events
            let collision_color = if state.frame_collisions > 0 { RED } else { WHITE };
            let boundary_color = if state.frame_boundary_hits > 0 { ORANGE } else { WHITE };
            draw_text(&format!("Frame collisions: {}", 
                state.frame_collisions), x, 240.0, 16.0, collision_color);
            draw_text(&format!("Frame boundaries: {}", 
                state.frame_boundary_hits), x, 260.0, 16.0, boundary_color);
            
            // Memory usage
            draw_text("MEMORY USAGE", x, 300.0, 18.0, YELLOW);
            let circle_size = std::mem::size_of::<determinisk_core::Circle>();
            let state_size = std::mem::size_of::<determinisk_core::SimulationState>();
            draw_text(&format!("Circle struct: {} bytes", 
                circle_size), x, 320.0, 16.0, WHITE);
            draw_text(&format!("State struct: {} bytes", 
                state_size), x, 340.0, 16.0, WHITE);
            draw_text(&format!("World RAM: {:.1} KB", 
                (circle_size * state.circles.len()) as f32 / 1024.0), x, 360.0, 16.0, WHITE);
        }
        
        // Controls
        let y = screen_height() - 220.0;
        draw_text("CONTROLS", 10.0, y, 24.0, YELLOW);
        draw_text("Space: Play/Pause", 10.0, y + 30.0, 20.0, WHITE);
        draw_text("←/→: Previous/Next frame", 10.0, y + 55.0, 20.0, WHITE);
        draw_text("R: Reset to start", 10.0, y + 80.0, 20.0, WHITE);
        let trail_color = if self.show_trails { GREEN } else { Color::new(0.5, 0.5, 0.5, 1.0) };
        draw_text("T: Toggle trails", 10.0, y + 105.0, 20.0, trail_color);
        let vel_color = if self.show_velocities { GREEN } else { Color::new(0.5, 0.5, 0.5, 1.0) };
        draw_text("V: Toggle velocities", 10.0, y + 130.0, 20.0, vel_color);
        let grid_color = if self.show_grid { GREEN } else { Color::new(0.5, 0.5, 0.5, 1.0) };
        draw_text("G: Toggle grid", 10.0, y + 155.0, 20.0, grid_color);
        let metrics_color = if self.show_metrics { GREEN } else { Color::new(0.5, 0.5, 0.5, 1.0) };
        draw_text("M: Toggle metrics", 10.0, y + 180.0, 20.0, metrics_color);
        
        // Playback status
        let status = if self.playing { "▶ PLAYING" } else { "⏸ PAUSED" };
        let status_color = if self.playing { GREEN } else { YELLOW };
        draw_text(status, screen_width() / 2.0 - 50.0, 35.0, 28.0, status_color);
    }
    
    pub async fn run(mut self) {
        // Set up camera to view the entire world
        let world_width = self.trace.input.world_width * PIXELS_PER_METER;
        let world_height = self.trace.input.world_height * PIXELS_PER_METER;
        
        loop {
            // Handle input
            self.handle_input();
            
            // Update frame
            if self.playing && self.current_frame < self.trace.states.len() - 1 {
                self.current_frame += 1;
            } else if self.playing && self.current_frame == self.trace.states.len() - 1 {
                self.playing = false; // Stop at end
            }
            
            // Clear screen
            clear_background(Color::new(0.1, 0.1, 0.15, 1.0));
            
            // Set camera to view the world properly
            // Calculate zoom to fit the world in the screen
            let zoom_x = screen_width() / world_width;
            let zoom_y = screen_height() / world_height;
            let zoom = zoom_x.min(zoom_y) * 0.9; // 0.9 to add some padding
            
            set_camera(&Camera2D {
                target: vec2(world_width / 2.0, world_height / 2.0),
                zoom: vec2(zoom / screen_width() * 2.0, zoom / screen_height() * 2.0),
                ..Default::default()
            });
            
            // Draw world
            if self.show_grid {
                self.draw_grid();
            }
            self.draw_boundaries();
            
            // Draw trails
            if self.show_trails {
                self.draw_trails();
            }
            
            // Draw circles
            let state = &self.trace.states[self.current_frame];
            for circle in &state.circles {
                self.draw_circle(circle, Color::new(0.5, 0.7, 1.0, 0.8));
                
                if self.show_velocities {
                    self.draw_velocity(circle);
                }
            }
            
            // Reset camera for UI
            set_default_camera();
            
            // Draw UI
            self.draw_ui();
            
            next_frame().await;
        }
    }
}

/// Visualize a simulation trace
pub async fn visualize_trace(trace: SimulationTrace) {
    let visualizer = Visualizer::new(trace);
    visualizer.run().await;
}

/// Visualize trace with live proof metrics updates
pub async fn visualize_trace_with_updates(
    trace: SimulationTrace,
    proof_metrics: std::sync::Arc<std::sync::Mutex<Option<ProofMetrics>>>,
) {
    let mut visualizer = Visualizer::new(trace);
    
    // Set up camera to view the entire world
    let world_width = visualizer.trace.input.world_width * PIXELS_PER_METER;
    let world_height = visualizer.trace.input.world_height * PIXELS_PER_METER;
    
    loop {
        // Update proof metrics if available
        if let Ok(metrics) = proof_metrics.lock() {
            visualizer.proof_metrics = metrics.clone();
        }
        
        // Check for input and update state
        visualizer.handle_input();
        
        // Update animation
        if visualizer.playing && visualizer.current_frame < visualizer.trace.states.len() - 1 {
            visualizer.current_frame += 1;
        }
        
        // Draw everything
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0));
        
        // Set camera to view the world properly
        // Calculate zoom to fit the world in the screen
        let zoom_x = screen_width() / world_width;
        let zoom_y = screen_height() / world_height;
        let zoom = zoom_x.min(zoom_y) * 0.9; // 0.9 to add some padding
        
        set_camera(&Camera2D {
            target: vec2(world_width / 2.0, world_height / 2.0),
            zoom: vec2(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0), // Negative Y for correct orientation
            ..Default::default()
        });
        
        if visualizer.show_grid {
            visualizer.draw_grid();
        }
        visualizer.draw_boundaries();
        
        if visualizer.show_trails {
            visualizer.draw_trails();
        }
        
        // Draw circles
        let state = &visualizer.trace.states[visualizer.current_frame];
        for circle in &state.circles {
            visualizer.draw_circle(circle, Color::new(0.5, 0.7, 1.0, 0.8));
            
            if visualizer.show_velocities {
                visualizer.draw_velocity(circle);
            }
        }
        
        // Reset camera for UI
        set_default_camera();
        
        // Draw UI
        visualizer.draw_ui();
        
        // Check for exit
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }
        
        next_frame().await;
    }
}