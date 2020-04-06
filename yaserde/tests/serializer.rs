#[macro_use]
extern crate yaserde_derive;

use std::io::Write;
use yaserde::ser::to_string;
use yaserde::YaSerialize;

macro_rules! convert_and_validate {
  ($model: expr, $content: expr) => {
    let data: Result<String, String> = to_string(&$model);
    assert_eq!(
      data,
      Ok(
        String::from($content)
          .split("\n")
          .map(|s| s.trim())
          .collect::<String>()
      )
    );
  };
}

#[test]
fn ser_basic() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    item: String,
  }

  let model = XmlStruct {
    item: "something".to_string(),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><item>something</item></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_list_of_items() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    items: Vec<String>,
  }

  let model = XmlStruct {
    items: vec!["something1".to_string(), "something2".to_string()],
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><items>something1</items><items>something2</items></base>";
  convert_and_validate!(model, content);

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStructOfStruct {
    items: Vec<SubStruct>,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "items")]
  pub struct SubStruct {
    field: String,
  }

  let model2 = XmlStructOfStruct {
    items: vec![
      SubStruct {
        field: "something1".to_string(),
      },
      SubStruct {
        field: "something2".to_string(),
      },
    ],
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><items><field>something1</field></items><items><field>something2</field></items></base>";
  convert_and_validate!(model2, content);
}

#[test]
fn ser_attributes() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute)]
    item: String,
    sub: SubStruct,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "sub")]
  pub struct SubStruct {
    #[yaserde(attribute)]
    subitem: String,
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct {
        subitem: "".to_string(),
      }
    }
  }

  assert_eq!(
    SubStruct::default(),
    SubStruct {
      subitem: "".to_string()
    }
  );

  let model = XmlStruct {
    item: "something".to_string(),
    sub: SubStruct {
      subitem: "sub-something".to_string(),
    },
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base item=\"something\"><sub subitem=\"sub-something\" /></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_attributes_complex() {
  mod other_mod {
    use super::*;

    #[derive(YaSerialize, PartialEq, Debug)]
    pub enum AttrEnum {
      #[yaserde(rename = "variant 1")]
      Variant1,
      #[yaserde(rename = "variant 2")]
      Variant2,
    }

    impl Default for AttrEnum {
      fn default() -> AttrEnum {
        AttrEnum::Variant1
      }
    }
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  pub struct Struct {
    #[yaserde(attribute)]
    attr_option_string: Option<std::string::String>,
    #[yaserde(attribute)]
    attr_option_enum: Option<other_mod::AttrEnum>,
  }

  impl Default for Struct {
    fn default() -> Struct {
      Struct {
        attr_option_string: None,
        attr_option_enum: None,
      }
    }
  }

  convert_and_validate!(
    Struct {
      attr_option_string: None,
      attr_option_enum: None,
    },
    r#"
    <?xml version="1.0" encoding="utf-8"?>
    <Struct />
    "#
  );

  convert_and_validate!(
    Struct {
      attr_option_string: Some("some value".to_string()),
      attr_option_enum: Some(other_mod::AttrEnum::Variant2),
    },
    r#"
    <?xml version="1.0" encoding="utf-8"?>
    <Struct attr_option_string="some value" attr_option_enum="variant 2" />
    "#
  );
}

#[test]
fn ser_rename() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename = "Item")]
    item: String,
    #[yaserde(rename = "sub")]
    sub_struct: SubStruct,
    #[yaserde(rename = "maj.min.bug")]
    version: String,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "sub")]
  pub struct SubStruct {
    #[yaserde(attribute, rename = "sub_item")]
    subitem: String,
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct {
        subitem: "".to_string(),
      }
    }
  }

  assert_eq!(
    SubStruct::default(),
    SubStruct {
      subitem: "".to_string()
    }
  );

  let model = XmlStruct {
    item: "something".to_string(),
    sub_struct: SubStruct {
      subitem: "sub_something".to_string(),
    },
    version: "2.0.2".into(),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base Item=\"something\"><sub sub_item=\"sub_something\" /><maj.min.bug>2.0.2</maj.min.bug></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_text_content_with_attributes() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    #[yaserde(attribute, rename = "Item")]
    item: String,
    #[yaserde(rename = "sub")]
    sub_struct: SubStruct,
  }

  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "sub")]
  pub struct SubStruct {
    #[yaserde(attribute, rename = "sub_item")]
    subitem: String,
    #[yaserde(text)]
    text: String,
  }

  impl Default for SubStruct {
    fn default() -> SubStruct {
      SubStruct {
        subitem: "".to_string(),
        text: "".to_string(),
      }
    }
  }

  assert_eq!(
    SubStruct::default(),
    SubStruct {
      subitem: "".to_string(),
      text: "".to_string(),
    }
  );

  let model = XmlStruct {
    item: "something".to_string(),
    sub_struct: SubStruct {
      subitem: "sub_something".to_string(),
      text: "text_content".to_string(),
    },
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base Item=\"something\"><sub sub_item=\"sub_something\">text_content</sub></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_name_issue_21() {
  #[derive(YaSerialize, PartialEq, Debug)]
  #[yaserde(root = "base")]
  pub struct XmlStruct {
    name: String,
  }

  let model = XmlStruct {
    name: "something".to_string(),
  };

  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><base><name>something</name></base>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_custom() {
  #[derive(Default, PartialEq, Debug, YaSerialize)]
  struct Date {
    #[yaserde(rename = "Year")]
    year: i32,
    #[yaserde(rename = "Month")]
    month: i32,
    #[yaserde(rename = "Day")]
    day: Day,
  }

  #[derive(Default, PartialEq, Debug)]
  struct Day {
    value: i32,
  }

  impl YaSerialize for Day {
    fn serialize<W: Write>(&self, writer: &mut yaserde::ser::Serializer<W>) -> Result<(), String> {
      let _ret = writer.write(xml::writer::XmlEvent::start_element("DoubleDay"));
      let _ret = writer.write(xml::writer::XmlEvent::characters(
        &(self.value * 2).to_string(),
      ));
      let _ret = writer.write(xml::writer::XmlEvent::end_element());
      Ok(())
    }
  }

  let model = Date {
    year: 2020,
    month: 1,
    day: Day { value: 5 },
  };
  let content = "<?xml version=\"1.0\" encoding=\"utf-8\"?><Date><Year>2020</Year><Month>1</Month><DoubleDay>10</DoubleDay></Date>";
  convert_and_validate!(model, content);
}

#[test]
fn ser_flatten() {
  #[derive(Default, PartialEq, Debug, YaSerialize)]
  struct DateTime {
    #[yaserde(flatten)]
    date: Date,
    time: String,
    #[yaserde(flatten)]
    kind: DateKind,
  }

  #[derive(Default, PartialEq, Debug, YaSerialize)]
  struct Date {
    year: i32,
    month: i32,
    day: i32,
    #[yaserde(flatten)]
    extra: Extra,
    #[yaserde(flatten)]
    optional_extra: Option<OptionalExtra>,
  }

  #[derive(Default, PartialEq, Debug, YaSerialize)]
  pub struct Extra {
    week: i32,
    century: i32,
  }

  #[derive(Default, PartialEq, Debug, YaSerialize)]
  pub struct OptionalExtra {
    lunar_day: i32,
  }

  #[derive(PartialEq, Debug, YaSerialize)]
  pub enum DateKind {
    #[yaserde(rename = "holidays")]
    Holidays(Vec<String>),
    #[yaserde(rename = "working")]
    Working,
  }

  impl Default for DateKind {
    fn default() -> Self {
      DateKind::Working
    }
  };

  let model = DateTime {
    date: Date {
      year: 2020,
      month: 1,
      day: 1,
      extra: Extra {
        week: 1,
        century: 21,
      },
      optional_extra: Some(OptionalExtra { lunar_day: 1 }),
    },
    time: "10:40:03".to_string(),
    kind: DateKind::Holidays(vec![
      "New Year's Day".into(),
      "Novy God Day".into(),
      "Polar Bear Swim Day".into(),
    ]),
  };

  let content = r#"
    <?xml version="1.0" encoding="utf-8"?>
    <DateTime>
      <year>2020</year>
      <month>1</month>
      <day>1</day>
      <week>1</week>
      <century>21</century>
      <lunar_day>1</lunar_day>
      <time>10:40:03</time>
      <holidays>New Year's Day</holidays>
      <holidays>Novy God Day</holidays>
      <holidays>Polar Bear Swim Day</holidays>
    </DateTime>"#;

  convert_and_validate!(model, content);
}
