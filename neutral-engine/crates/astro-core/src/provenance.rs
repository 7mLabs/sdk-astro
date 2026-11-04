//! Optional caller declarations for externally supplied geometry, never inferred.
use super::*;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct SourceContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    zodiac: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ayanamsa: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reference_frame: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    coordinates: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    house_system: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    source_version: Option<String>,
}

impl SourceContext {
    fn validate(&self) -> Result<(), String> {
        let fields = [
            &self.zodiac,
            &self.ayanamsa,
            &self.reference_frame,
            &self.coordinates,
            &self.house_system,
            &self.source_id,
            &self.source_version,
        ];
        if fields.iter().all(|v| v.is_none()) {
            return Err("sourceContext must contain a nonempty declaration".into());
        }
        if fields
            .iter()
            .filter_map(|v| v.as_ref())
            .any(|s| s.trim().is_empty() || s.len() > 128)
        {
            return Err("sourceContext values must be nonblank and at most 128 UTF-8 bytes".into());
        }
        if self
            .zodiac
            .as_deref()
            .is_some_and(|s| !["tropical", "sidereal"].contains(&s))
        {
            return Err("sourceContext.zodiac must be tropical or sidereal".into());
        }
        if self
            .coordinates
            .as_deref()
            .is_some_and(|s| !["geocentric", "topocentric", "heliocentric"].contains(&s))
        {
            return Err(
                "sourceContext.coordinates must be geocentric, topocentric or heliocentric".into(),
            );
        }
        if self.ayanamsa.is_some() && self.zodiac.as_deref() != Some("sidereal") {
            return Err("sourceContext.ayanamsa requires zodiac sidereal".into());
        }
        Ok(())
    }
}

// JSON-only additions leave the public Rust Request struct unchanged, including
// existing downstream struct literals and calculate(Request) calls.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct GeometryInput {
    operation: String,
    positions: Vec<Position>,
    #[serde(default)]
    other_positions: Option<Vec<Position>>,
    #[serde(default)]
    houses: Option<Vec<f64>>,
    #[serde(default)]
    other_houses: Option<Vec<f64>>,
    #[serde(default, deserialize_with = "numbers::optional_u32")]
    harmonic: Option<u32>,
    #[serde(default = "default_rules")]
    aspect_rules: Vec<AspectRule>,
    #[serde(default, deserialize_with = "natal::present")]
    source_context: Option<SourceContext>,
    #[serde(default, deserialize_with = "natal::present")]
    other_source_context: Option<SourceContext>,
}

fn declaration(context: Option<SourceContext>) -> Value {
    json!({"status":if context.is_some() { "callerDeclared" } else { "unspecified" },
        "verified":false,"context":context})
}

pub(super) fn unspecified(synastry: bool) -> Value {
    let mut value = json!({"positions":declaration(None)});
    if synastry {
        value["otherPositions"] = declaration(None);
    }
    value
}

impl GeometryInput {
    pub(super) fn calculate(self) -> Result<Value, String> {
        for context in [&self.source_context, &self.other_source_context]
            .into_iter()
            .flatten()
        {
            context.validate()?;
        }
        if self.other_source_context.is_some() && self.operation != "synastry" {
            return Err("otherSourceContext requires operation synastry".into());
        }
        let synastry = self.operation == "synastry";
        let mut provenance = json!({"positions":declaration(self.source_context)});
        if synastry {
            provenance["otherPositions"] = declaration(self.other_source_context);
        }
        let mut result = super::calculate(Request {
            operation: self.operation,
            positions: self.positions,
            other_positions: self.other_positions,
            houses: self.houses,
            other_houses: self.other_houses,
            harmonic: self.harmonic,
            aspect_rules: self.aspect_rules,
        })?;
        result["calculation"]["inputProvenance"] = provenance;
        Ok(result)
    }
}
