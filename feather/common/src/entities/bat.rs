// This file is @generated. Please do not edit.
use libcraft::EntityKind;
use quill::components::EntityKindComponent;
use quill::entities::Bat;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Bat).add(EntityKindComponent(EntityKind::Bat));
}