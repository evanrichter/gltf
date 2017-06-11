
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::json::{accessor, scene, Extras, Index, Root};
use v2::validation::{Error, JsonPath, Validate};

/// A keyframe animation.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Animation {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: AnimationExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// An array of channels, each of which targets an animation's sampler at a
    /// node's property.
    ///
    /// Different channels of the same animation must not have equal targets.
    pub channels: Vec<Channel>,
    
    /// Optional user-defined name for this object.
    pub name: Option<String>,
    
    /// An array of samplers that combine input and output accessors with an
    /// interpolation algorithm to define a keyframe graph (but not its target).
    pub samplers: Vec<Sampler>,
}

/// Extension specific data for `Animation`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct AnimationExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// Targets an animation's sampler at a node's property.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Channel {
    /// The index of a sampler in this animation used to compute the value for the target.
    pub sampler: Index<Sampler>,
    
    /// The index of the node and TRS property to target.
    pub target: Target,
    
    /// Extension specific data.
    #[serde(default)]
    pub extensions: ChannelExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
}

/// Extension specific data for `Channel`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct ChannelExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// The index of the node and TRS property that an animation channel targets.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Target {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: TargetExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// The index of the node to target.
    pub node: Index<scene::Node>,
    
    /// The name of the node's TRS property to modify or the 'weights' of the
    /// morph targets it instantiates.
    pub path: String,
}

/// Extension specific data for `Target`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct TargetExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

/// Defines a keyframe graph but not its target.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
#[serde(deny_unknown_fields)]
pub struct Sampler {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: SamplerExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// The index of an accessor containing keyframe input values, e.g., time.
    pub input: Index<accessor::Accessor>,
    
    /// The interpolation algorithm.
    #[serde(default = "sampler_interpolation_default")]
    pub interpolation: String,
    
    /// The index of an accessor containing keyframe output values.
    pub output: Index<accessor::Accessor>,
}

fn sampler_interpolation_default() -> String {
    "LINEAR".to_string()
}

/// Extension specific data for `Sampler`.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct SamplerExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

impl Validate for Animation {
    fn validate<F>(&self, root: &Root, path: JsonPath, mut report: &mut F)
        where F: FnMut(Error)
    {
        self.samplers.validate(root, path.field("samplers"), report);
        self.channels.validate(root, path.field("channels"), report);
        for (index, channel) in self.channels.iter().enumerate() {
            if channel.sampler.value() as usize >= self.samplers.len() {
                let field = format!("channels[{}].sampler", index);
                report(Error::index_out_of_bounds(path.field(&field)));
            }
        }
    }
}

impl Validate for Channel {
    fn validate<F>(&self, _root: &Root, _path: JsonPath, _report: &mut F)
        where F: FnMut(Error)
    {
        // nop
    }
}
