use crate::parser::types::{
    Directive, DirectiveValue, Interface, InterfaceProperty, PropertyValue,
};

pub fn to_js_obj(interface: Interface) -> String {
    let mut str = format!(
        "Component.defaultProps = {{}}
            
Component.uiData = {{
    name: \"{}\",
    directives: [],
    properties: [],
}}",
        interface.name
    );

    str = str.replace(
        "Component.defaultProps = {}",
        &do_defaults(&interface.properties),
    );
    str = str.replace("directives: [],", &do_directives(&interface.directives, 8));
    str = str.replace("properties: [],", &do_properties(&interface.properties));

    str
}

fn do_properties(properties: &[InterfaceProperty]) -> String {
    let mut str = "properties: [".to_owned();

    properties.iter().for_each(|p| {
        let v = match &p.value {
            PropertyValue::String(s) => format!("\"{}\"", s),
            PropertyValue::Number(n) => n.to_string(),
        };

        str += &format!(
            "
        {{
            key: \"{}\",
            value: {},
            directives: []
        }},",
            p.key, v
        );

        str = str.replace("directives: []", &do_directives(&p.directives, 16));
    });

    if str.ends_with('[') {
        str += "],";
    } else {
        str += "\n    ],";
    }

    str
}

fn do_directives(directives: &[Directive], tabsize: u8) -> String {
    let mut str = "directives: [".to_owned();

    directives.iter().for_each(|d| {
        let v = match &d.value {
            DirectiveValue::String(s) => format!("\"{}\"", s),
            DirectiveValue::Number(n) => n.to_string(),
        };

        str += &format!(
            "
{}{{ key: \"{}\", value: {} }},",
            tab(tabsize),
            d.key,
            v
        );
    });

    if str.ends_with('[') {
        str += "],";
    } else {
        str += &format!("\n{}],", tab(tabsize - 4));
    }

    str
}

fn do_defaults(properties: &[InterfaceProperty]) -> String {
    let mut str = "Component.defaultProps = {".to_owned();

    properties.iter().for_each(|p| {
        let v = match &p
            .directives
            .iter()
            .filter(|&d| d.key == "default")
            .collect::<Vec<_>>()[0]
            .value
        {
            DirectiveValue::String(s) => format!("\"{}\"", s),
            DirectiveValue::Number(n) => n.to_string(),
        };

        str += &format!(
            "
    {}: {},",
            p.key, v
        );
    });

    if str.ends_with('[') {
        str += "}";
    } else {
        str += "\n}";
    }

    str
}

fn tab(tabsize: u8) -> String {
    let mut str = "".to_owned();

    for _ in 0..tabsize {
        str += " ";
    }

    str
}
