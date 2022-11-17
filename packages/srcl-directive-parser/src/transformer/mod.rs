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
            PropertyValue::String(s) => {
                if s.contains('"') {
                    format!("'{}'", s)
                } else {
                    format!("\"{}\"", s)
                }
            }
            PropertyValue::Expression(e) => e.to_string(),
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
            DirectiveValue::Boolean(b) => b.to_string(),
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
        str += "]";
    } else {
        str += &format!("\n{}],", tab(tabsize - 4));
    }

    str
}

fn do_defaults(properties: &[InterfaceProperty]) -> String {
    let mut str = "Component.defaultProps = {".to_owned();

    properties.iter().for_each(|p| {
        let v_arr = &p
            .directives
            .iter()
            .filter(|&d| d.key == "default")
            .collect::<Vec<_>>();

        if p.directives.len() > 0 {
            if v_arr.len() <= 0 {
                panic!("missing \"default\" directive vor property \"{}\"", p.key);
            }

            let v = match &v_arr[0].value {
                DirectiveValue::String(s) => format!("\"{}\"", s),
                DirectiveValue::Number(n) => n.to_string(),
                DirectiveValue::Boolean(b) => b.to_string(),
            };

            str += &format!(
                "
    {}: {},",
                p.key, v
            );
        }
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
