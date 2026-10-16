use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    Exact,
    Estimated,
    Unavailable,
    NotApplicable,
}

/// A technical value that always says how reliable it is.
/// `unavailable` is never the same thing as zero.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TechnicalValue<T> {
    pub value: Option<T>,
    pub confidence: Confidence,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

impl<T> TechnicalValue<T> {
    pub fn exact(value: T) -> Self {
        Self::known(value, Confidence::Exact)
    }

    pub fn estimated(value: T, method: impl Into<String>) -> Self {
        Self {
            method: Some(method.into()),
            ..Self::known(value, Confidence::Estimated)
        }
    }

    pub fn unavailable() -> Self {
        Self::empty(Confidence::Unavailable)
    }

    pub fn not_applicable() -> Self {
        Self::empty(Confidence::NotApplicable)
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    fn known(value: T, confidence: Confidence) -> Self {
        Self {
            value: Some(value),
            confidence,
            unit: None,
            method: None,
        }
    }

    fn empty(confidence: Confidence) -> Self {
        Self {
            value: None,
            confidence,
            unit: None,
            method: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_has_no_value() {
        let triangles = TechnicalValue::<u64>::unavailable();
        assert_eq!(triangles.value, None);
        assert_eq!(triangles.confidence, Confidence::Unavailable);
    }

    #[test]
    fn zero_is_a_real_value() {
        let triangles = TechnicalValue::exact(0_u64);
        assert_eq!(triangles.value, Some(0));
        assert_eq!(triangles.confidence, Confidence::Exact);
    }

    #[test]
    fn estimated_keeps_its_method() {
        let triangles = TechnicalValue::estimated(12_u64, "obj triangulation");
        assert_eq!(triangles.method.as_deref(), Some("obj triangulation"));
    }

    #[test]
    fn serializes_without_empty_fields() {
        let duration = TechnicalValue::exact(2.5_f64).with_unit("s");
        let json = serde_json::to_value(&duration).unwrap();
        assert_eq!(
            json,
            serde_json::json!({ "value": 2.5, "confidence": "exact", "unit": "s" })
        );

        let animations = TechnicalValue::<u32>::not_applicable();
        let json = serde_json::to_value(&animations).unwrap();
        assert_eq!(
            json,
            serde_json::json!({ "value": null, "confidence": "not_applicable" })
        );
    }
}
