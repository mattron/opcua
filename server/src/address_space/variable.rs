use address_space::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub base: Base,
}

node_impl!(Variable);

impl Variable {
    pub fn new(node_id: &NodeId, browse_name: &str, display_name: &str, value: &DataValue) -> Variable {
        // Mandatory
        let historizing = false;
        let access_level = 0;
        let user_access_level = 0;
        let value_rank = -1; // TODO if value is an array, maybe this and array dimensions should be explicitly set
        let attributes = vec![
            (AttributeId::UserAccessLevel, Variant::Byte(user_access_level)),
            (AttributeId::AccessLevel, Variant::Byte(access_level)),
            (AttributeId::ValueRank, Variant::Int32(value_rank)),
            (AttributeId::Historizing, Variant::Boolean(historizing)),
        ];

        // Optional
        // attrs.push(Attribute::MinimumSamplingInterval(0));
        // attrs.push(Attribute::ArrayDimensions(1));

        let properties = vec![];
        let mut result = Variable {
            base: Base::new(NodeClass::Variable, node_id, browse_name, display_name, attributes, properties),
        };
        result.base.set_attribute(AttributeId::Value, value.clone());
        result
    }

    pub fn new_array(node_id: &NodeId, browse_name: &str, display_name: &str, value: &DataValue, dimensions: &[Int32]) -> Variable {
        let mut variable = Variable::new(node_id, browse_name, display_name, value);
        // An array has a value rank equivalent to the number of dimensions and an ArrayDimensions array
        let now = DateTime::now();
        variable.base.set_attribute_value(AttributeId::ValueRank, Variant::Int32(dimensions.len() as Int32), &now, &now);
        variable.base.set_attribute_value(AttributeId::ArrayDimensions, Variant::from_i32_array(dimensions), &now, &now);
        variable
    }

    pub fn value(&self) -> DataValue {
        if let &Some(ref attribute) = &self.base.attributes[Base::attribute_idx(AttributeId::Value)] {
            attribute.clone()
        } else {
            panic!("Variable value is missing");
        }
    }

    /// Sets the variable's value
    pub fn set_value(&mut self, value: DataValue)  {
        // Value is directly set - it's a datavalue
        self.base.attributes[Base::attribute_idx(AttributeId::Value)] = Some(value);
    }

    pub fn access_level(&self) -> Byte {
        find_attribute_value_mandatory!(&self.base, AccessLevel, Byte)
    }

    pub fn user_access_level(&self) -> Byte {
        find_attribute_value_mandatory!(&self.base, UserAccessLevel, Byte)
    }

    pub fn value_rank(&self) -> Int32 {
        find_attribute_value_mandatory!(&self.base, ValueRank, Int32)
    }

    pub fn historizing(&self) -> Boolean {
        find_attribute_value_mandatory!(&self.base, Historizing, Boolean)
    }
}