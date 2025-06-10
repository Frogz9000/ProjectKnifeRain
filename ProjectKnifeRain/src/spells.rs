//we'll probably want to move this to a folder but i don;t know where yet
use bevy::prelude::*;
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionEvent, Damping, ExternalImpulse, GravityScale, RigidBody, Sleeping, Velocity};

pub struct SpellsPlugin;
impl Plugin for SpellsPlugin{
    fn build(&self, app: &mut App){
        app.add_event::<CastSpellEvent>();
        app.add_systems(Update,(handle_fireball_event,fireball_collision_process));
        //add other handlers here
    }
}
pub enum SpellTypes {
    Fireball,
    IceStorm,
    //more to add later
}
#[derive(Event)]
pub struct CastSpellEvent {
    pub spell_type: SpellTypes,
    pub origin: Vec3,
    pub direction: Vec3,
    //potentially add more modifier
    //such as speed, arch, flight pattern, etc
}

impl CastSpellEvent{
pub fn origin_and_direction_to_transform(&self) -> Transform{
    return Transform::from_translation(self.origin)
                     .looking_to(self.direction.normalize_or_zero(), Vec3::Y);
}
}

#[derive(Component)]
pub struct Fireball;

//have these const for now, potentially pass in through fireball so spell strength can be modified
const KNOCKBACK_STRENGTH: f32 = 5.0;
const KNOCKBACK_RADIUS: f32 = 3.0;
const FIREBALL_SPEED: f32 = 0.1;
const FIREBALL_COLLIDER_RADIUS: f32 = 0.5;

pub fn handle_fireball_event(
    mut command: Commands,
    mut fireball_event: EventReader<CastSpellEvent>,
){ 
    for fireball in fireball_event.read(){
        if let SpellTypes::Fireball = fireball.spell_type{
            let transform = fireball.origin_and_direction_to_transform();
            command.spawn((
            RigidBody::Dynamic,
            GravityScale(0.0),//sets the object to be unaffected by gravity
            transform,
            Velocity{
                linvel: transform.forward() * FIREBALL_SPEED,
                angvel: Vec3::ZERO,
            },
            Collider::ball(FIREBALL_COLLIDER_RADIUS),
            ActiveEvents::COLLISION_EVENTS,
            Fireball,
            // Add this to prevent physics from slowing down the fireball
            Damping {
                linear_damping: 0.0,
                angular_damping: 0.0,
            },
            Sleeping::disabled(),
        ));
        }
    }
}
//RIGHT NOW WE QUERY ALL KNOCKBACKABLE, AS GAME GETS LARGER WE MAY NEED TO TRANSITION TO USING bevy_spatial TO SPATIAL QUERY ONLY NEArBY OBJECTS
//OR ADD ROOM ID TAGS TO OBJECTS AND HAVE THIS ONLY CHECK THE ITEMS IN THE ROOM YOU CAST IN BY FILTERING ID <- CAN CAUSE WEIRD ISSUES WITH SHOOTING THROUGH A DOOR
fn fireball_collision_process(
    mut command: Commands,
    mut collision_event: EventReader<CollisionEvent>,//this -> ActiveEvents::COLLISION_EVENTS makes rapier trigger collision events for that objcet
    fireballs: Query<(), With<Fireball>>,//get all fireballs to see which is colliding
    fireball_query: Query<&GlobalTransform, With<Fireball>>,
    mut knockback_entities: Query<(&mut ExternalImpulse, &GlobalTransform, &Velocity)>//get all things that can be knocked back to see if they are close enough to fireball
){
    //loop thorugh all collision events to make sure a fireball was involved
    for event in collision_event.read(){
        if let CollisionEvent::Started(fireball_entity,hit_entity , _) = event {
            let (fireball_entity, _hit_entity) = if fireballs.get(*fireball_entity).is_ok() {
                (*fireball_entity, *hit_entity)
            } else if fireballs.get(*hit_entity).is_ok() {
                (*hit_entity, *fireball_entity)
            } else {
                continue; // not a fireball collision
            };
            //get fireball position
            let Ok(fireball_transform) = fireball_query.get(fireball_entity) else{continue;};
            let fireball_pos = fireball_transform.translation();
            //generate knockback impulse
            for (mut impulse, hit_transform,velocity) in knockback_entities.iter_mut(){
                let hit_pos = hit_transform.translation();
                let direction = hit_pos - fireball_pos;
                let distance = direction.length();
                if distance <= KNOCKBACK_RADIUS {
                    let collision_normal = direction.normalize_or_zero();
                    let mut knockback_dir = collision_normal;
                    let is_falling = velocity.linvel.y > 0.0;
                    if is_falling {
                        let fall_cancel = -velocity.linvel.y;
                        knockback_dir.y += fall_cancel/KNOCKBACK_STRENGTH;//need to divide by knockback strenght to cancel mult at end
                    }
                    if collision_normal.y > 0.5 {
                        //hit from below -> strong upward launch
                        knockback_dir.y += 1.5;
                    } else if collision_normal.y < -0.3 {
                        //hit from above => downward launch
                        knockback_dir.y = -1.0;
                    } else {
                        //front,back,side hits -> emphasize horizontal
                        knockback_dir.x *= 1.5;
                        knockback_dir.z *= 1.5;
                        knockback_dir.y = 0.3; //slight upwards
                    }
                    //distance falloff
                    let falloff = 1.0 - (distance / KNOCKBACK_RADIUS);
                    impulse.impulse = knockback_dir.normalize_or_zero() * KNOCKBACK_STRENGTH * falloff;
                }
            }
            //destroy fireball after processing
            command.entity(fireball_entity).despawn();
        }   
    }
}
