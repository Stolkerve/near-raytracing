use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen};
use image::{RgbaImage, Rgba};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct Vec3 {
    pub x:f32, pub y:f32, pub z:f32
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


impl std::ops::Sub<&Vec3> for  Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Sub<Vec3> for  Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<&f32> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, other: &f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Mul<f32> for &Vec3 {
    type Output = Vec3;
    #[inline(always)]
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Vec3 {
        Vec3 { 
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

#[near_bindgen]
impl Vec3 {
    pub fn length(&self) -> f32 {
        return f32::sqrt((self.x * self.x) + (self.y * self.y) + (self.z * self.z));
    }

    pub fn normalize(&self) -> Vec3 {
        let length = self.length();
        return Vec3 {
            x: self.x * 1.0 / length,
            y: self.y * 1.0 / length,
            z: self.z * 1.0 / length
        }
    }

    pub fn dot_product(&self, v: &Vec3) -> f32 {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Camera {
    pub x: Vec3, // x es un vector que apunta a la derecha
    pub y: Vec3, // y apunta a arriba
    pub z: Vec3, // z apunta hacia delante
    pub ray_dir: Vec3, pub ray_origin: Vec3
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Sphere {
    color: Vec3, position: Vec3,
    reflectivity: f32, radius: f32
}

#[near_bindgen]
impl Sphere {
    pub fn is_hit_by_ray( &mut self,
        incoming_ray_origin: &Vec3,
        incoming_ray_direction: &Vec3,
        outgoing_ray_origin: &mut Vec3,
        outgoing_ray_direction: &mut Vec3,
        hit_distance: &mut f32,
        hit_color: &mut Vec3,
        object_reflectivity: &mut f32) -> bool
    {
        let p: Vec3 = self.position - incoming_ray_origin;
        let threshold: f32 = f32::sqrt(p.dot_product(&p) - self.radius * self.radius);
        let b: f32 = p.dot_product(incoming_ray_direction);

        if b > threshold {
            // HIT
            let s: f32 = f32::sqrt(p.dot_product(&p) - b * b);
            let t: f32 = f32::sqrt(self.radius * self.radius - s * s);
            *hit_distance = b - t;

            if *hit_distance < 1e-3
            {
                return false;
            }
            let hit_distance_copy = *hit_distance;
            *outgoing_ray_origin = incoming_ray_origin + &((incoming_ray_direction * hit_distance_copy));
            let normal: Vec3 = (-p + incoming_ray_direction * hit_distance_copy).normalize();

            *outgoing_ray_direction = (incoming_ray_direction + &(normal.normalize() * (normal.normalize().dot_product(&(-(*incoming_ray_direction)))) * 2.0)).normalize();

            *hit_color = self.color;
            *object_reflectivity = self.reflectivity;

            return true;
        } else {
            // NO HIT
            return false;
        }
    }
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Raytracing {
}

#[near_bindgen]
impl Raytracing {
    pub fn mint(&self, width: i32, height: i32, sky_color: Vec3) -> String{
        let mut camera = Camera {
            x: Vec3{x: 0.002, y: 0.0, z: 0.0 },
            y: Vec3{x: 0.0, y: 0.002, z: 0.0 },
            z: Vec3{x: 0.0, y: 0.0, z: 1.0 },
            ray_dir: Vec3{x: 0.0, y: 0.0, z: 0.0},
            ray_origin: Vec3{x: 0.0, y: 1.0, z: -4.0 }
        };

        let mut shapes: Vec<Sphere> = Vec::new();
        shapes.push(Sphere{
            position: Vec3{x: -0.5, y: 1.0, z: 0.0},
            radius: 0.5,
            reflectivity: 0.05,
            color: Vec3{x: 255.0 / 255.0, y: 165.2 / 255.0, z: 0.0}
        });
        shapes.push(Sphere{
            position: Vec3{x: 0.5, y: 1.0, z: 0.0},
            radius: 0.5,
            reflectivity: 1.00,
            color: Vec3{x: 255.0 / 255.0, y: 165.2 / 255.0, z: 0.0}
        });

        let mut img = RgbaImage::new(width as u32, height as u32);
        for y in -height / 2 .. (height / 2) + 1 {
            for x in  -width / 2 .. (width / 2) + 1 {
                camera.ray_dir = ( (camera.x * (x as f32 - 0.5)) + (camera.y * (y  as f32 - 0.5)) + camera.z ).normalize() ;
                camera.ray_origin =  Vec3{x: 0.0, y: 1.0, z: -4.0 };
                let mut final_color = Vec3{x: 0.0, y: 0.0, z: 0.0};
                let mut ray_hit_at = Vec3{x: 0.0, y: 0.0, z: 0.0};
                let mut ray_bounced_direction = Vec3{x: 0.0, y: 0.0, z: 0.0};
                let mut distance_to_hit: f32 = 0.0;
                let mut reflectivity_at_hit: f32 = 0.0;
                let mut ray_energy_left: f32= 1.0;
                
                for _bounce in 0 .. 100
                {
                    let mut color = Vec3{x: 0.0, y: 0.0, z: 0.0};
                    let mut an_object_was_hit = false;
                    let mut min_hit_distance = f32::MAX;
                    let mut closest_object_ptr: *mut Sphere = std::ptr::null_mut();
                    for s in &mut shapes
                    {
                        if s.is_hit_by_ray(
                            &camera.ray_origin,
                            &camera.ray_dir,
                            &mut ray_hit_at,
                            &mut ray_bounced_direction,
                            &mut distance_to_hit,
                            &mut color,
                            &mut reflectivity_at_hit)
                        {
                            an_object_was_hit = true;
                            if distance_to_hit < min_hit_distance
                            {
                                min_hit_distance = distance_to_hit;
                                closest_object_ptr = s;
                                // assert_eq!(true, !closest_object_ptr.is_null());
                            }
                        }
                    }
                    unsafe
                    {
                        if !closest_object_ptr.is_null()
                        {
                            if (*closest_object_ptr).is_hit_by_ray(
                                &camera.ray_origin,
                                &camera.ray_dir,
                                &mut ray_hit_at,
                                &mut ray_bounced_direction,
                                &mut distance_to_hit,
                                &mut color,
                                &mut reflectivity_at_hit)
                            {
                                camera.ray_origin = ray_hit_at;
                                camera.ray_dir = ray_bounced_direction;
                            }
                        }
                        else
                        {
                            if (camera.ray_dir.y.floor() as i32) < 0
                            {
                                color = calculate_ground_color(&camera.ray_origin, &camera.ray_dir);
                                reflectivity_at_hit = 0.0;
                            }
                            else
                            {
                                color = calculate_sky_color(&camera.ray_dir, &(sky_color.normalize()));
                                reflectivity_at_hit = 0.0;
                            }
                        }
                    }

                    final_color = final_color + (color * (ray_energy_left * (1.0 - reflectivity_at_hit)));
                    ray_energy_left *= reflectivity_at_hit;
                    if ray_energy_left <= 0.0
                    {
                        break;
                    }
                }

                let r: u8 = (final_color.x * 255.0).floor() as u8; 
                let g: u8 = (final_color.y * 255.0).floor() as u8; 
                let b: u8 = (final_color.z * 255.0).floor() as u8;

                img.put_pixel((-x + (width / 2)) as u32, (-y + (height / 2)) as u32, Rgba([r, g, b, 255u8]));
            }
        }
        
        let img: image::DynamicImage = image::DynamicImage::ImageRgba8(img);
        let mut png_buf = vec![];
        img.write_to(&mut png_buf, image::ImageOutputFormat::Png);
        let mut output = String::from("data:image/png;base64,");
        output.push_str(base64::encode(&png_buf).as_str());
        return output;
    }
}

fn calculate_sky_color(ray_dir: &Vec3, color: &Vec3) -> Vec3{
    return Vec3{x: color.x, y: color.y, z: color.z} * f32::powf(1.0 - ray_dir.y, 2.0);
}

fn calculate_ground_color(ray_origin: &Vec3, ray_dir: &Vec3) -> Vec3
{
    let distance: f32 = -ray_origin.y / ray_dir.y;
    let x: f32 = ray_origin.x + ray_dir.x * distance;
    let z: f32 = ray_origin.z + ray_dir.z * distance;
    
    if i32::abs(x.floor() as i32) % 2 == i32::abs(z.floor() as i32) % 2
    {
        return Vec3{x: 0.0, y: 0.0, z: 0.0};
    } else {
        return Vec3{x: 1.0, y: 1.0, z: 1.0};
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn create_texture() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let contract = Raytracing{};
        let png_base64 = &contract.mint(255, 255, Vec3{x: 71.0 / 255.0, y: 164.0 / 255.0, z:245.0 / 255.0});
        println!("Imagen png en base 64: {}", png_base64);
        // confirm that we received 1 when calling get_num
        assert_ne!(0, png_base64.len());
    }

    #[test]
    fn vec3_add() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let v1 = Vec3 {x: 5.0, y: 5.0, z: 5.0};
        let v2 = Vec3 {x: 1.0, y: 2.0, z: 3.0};
        let v3 = v1 + v2;
        println!("Dos Vec3 sumandos: x:{}, y: {}, z: {}", v3.x, v3.y, v3.z);

        assert_eq!(6.0, v3.x);
        assert_eq!(7.0, v3.y);
        assert_eq!(8.0, v3.z);
    }

    #[test]
    fn vec3_sub() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let v1 = Vec3 {x: 5.0, y: 5.0, z: 5.0};
        let v2 = Vec3 {x: 1.0, y: 2.0, z: 3.0};
        let v3 = v1 - v2;
        println!("Dos Vec3 sumandos: x:{}, y: {}, z: {}", v3.x, v3.y, v3.z);

        assert_eq!(4.0, v3.x);
        assert_eq!(3.0, v3.y);
        assert_eq!(2.0, v3.z);
    }
    
    #[test]
    fn vec3_neg() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let v1 = Vec3 {x: 1.0, y: 2.0, z: 3.0};
        let v3 = -v1;
        println!("Dos Vec3 sumandos: x:{}, y: {}, z: {}", v3.x, v3.y, v3.z);
        println!("Longitud de la suma {}", v3.length());

        assert_eq!(-1.0, v3.x);
        assert_eq!(-2.0, v3.y);
        assert_eq!(-3.0, v3.z);
    }
}