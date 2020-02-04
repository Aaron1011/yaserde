use attribute::*;
use de::build_default_value::build_default_value;
use field_type::*;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use std::collections::BTreeMap;
use syn::DataStruct;
use syn::Ident;

pub fn parse(
  data_struct: &DataStruct,
  name: &Ident,
  root: &str,
  prefix: &Option<String>,
  namespaces: &BTreeMap<String, String>,
) -> TokenStream {
  let namespaces_matches: TokenStream = namespaces
    .iter()
    .map(|(p, ns)| {
      let str_ns = ns.as_str();
      if *prefix == Some(p.to_string()) {
        Some(quote!(#str_ns => {}))
      } else {
        None
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(TokenStream::new(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

  let variables: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let label = &get_value_label(&field.ident);
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => build_default_value(
          label,
          &quote! {String},
          &quote! {"".to_string()},
          &field_attrs.default,
        ),
        Some(FieldType::FieldTypeBool) => {
          build_default_value(label, &quote! {bool}, &quote! {false}, &field_attrs.default)
        }
        Some(FieldType::FieldTypeI8) => {
          build_default_value(label, &quote! {i8}, &quote! {0}, &field_attrs.default)
        }
        Some(FieldType::FieldTypeU8) => {
          build_default_value(label, &quote! {u8}, &quote! {0}, &field_attrs.default)
        }
        Some(FieldType::FieldTypeI16) => {
          build_default_value(label, &quote! {i16}, &quote! {0}, &field_attrs.default)
        }
        Some(FieldType::FieldTypeU16) => {
          build_default_value(label, &quote! {u16}, &quote! {0}, &field_attrs.default)
        }
        Some(FieldType::FieldTypeI32) => {
          build_default_value(label, &quote! {i32}, &quote! {0}, &field_attrs.default)
        }
        Some(FieldType::FieldTypeU32) => {
          build_default_value(label, &quote! {u32}, &quote! {0}, &field_attrs.default)
        }
        Some(FieldType::FieldTypeI64) => {
          build_default_value(label, &quote! {i64}, &quote! {0}, &field_attrs.default)
        }
        Some(FieldType::FieldTypeU64) => {
          build_default_value(label, &quote! {u64}, &quote! {0}, &field_attrs.default)
        }
        Some(FieldType::FieldTypeF32) => {
          build_default_value(label, &quote! {f32}, &quote! {0.0}, &field_attrs.default)
        }
        Some(FieldType::FieldTypeF64) => {
          build_default_value(label, &quote! {f64}, &quote! {0.0}, &field_attrs.default)
        }
        Some(FieldType::FieldTypeStruct { struct_name }) => build_default_value(
          label,
          &quote! {#struct_name},
          &quote! {#struct_name::default()},
          &field_attrs.default,
        ),
        Some(FieldType::FieldTypeOption { .. }) => {
          if let Some(d) = &field_attrs.default {
            let default_function = Ident::new(&d, Span::call_site());

            Some(quote! {
              #[allow(unused_mut, non_snake_case, non_camel_case_types)]
              let mut #label = #default_function();
            })
          } else {
            Some(quote! {
              #[allow(unused_mut, non_snake_case, non_camel_case_types)]
              let mut #label = None;
            })
          }
        }
        Some(FieldType::FieldTypeVec { data_type }) => {
          let dt = Box::into_raw(data_type);
          match unsafe { dt.as_ref() } {
            Some(&FieldType::FieldTypeString) => build_default_value(
              label,
              &quote! {Vec<String>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeBool) => build_default_value(
              label,
              &quote! {Vec<bool>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeI8) => build_default_value(
              label,
              &quote! {Vec<i8>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeU8) => build_default_value(
              label,
              &quote! {Vec<u8>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeI16) => build_default_value(
              label,
              &quote! {Vec<i16>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeU16) => build_default_value(
              label,
              &quote! {Vec<u16>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeI32) => build_default_value(
              label,
              &quote! {Vec<i32>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeU32) => build_default_value(
              label,
              &quote! {Vec<u32>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeI64) => build_default_value(
              label,
              &quote! {Vec<i64>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeU64) => build_default_value(
              label,
              &quote! {Vec<u64>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeF32) => build_default_value(
              label,
              &quote! {Vec<f32>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeF64) => build_default_value(
              label,
              &quote! {Vec<f64>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeStruct { ref struct_name }) => build_default_value(
              label,
              &quote! {Vec<#struct_name>},
              &quote! {vec![]},
              &field_attrs.default,
            ),
            Some(&FieldType::FieldTypeOption { .. }) | Some(&FieldType::FieldTypeVec { .. }) => {
              unimplemented!();
            }
            None => {
              unimplemented!();
            }
          }
        }
        None => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(TokenStream::new(), |mut sum, val| {
      sum.append_all(val);
      sum
    });

  let field_visitors: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      let label_name = field_attrs
        .rename
        .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string());

      let visitor_label = build_visitor_ident(&label_name, None);

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => {
          build_declare_visitor(&quote! {String}, &quote! {visit_str}, &visitor_label)
        }
        Some(FieldType::FieldTypeBool) => {
          build_declare_visitor(&quote! {bool}, &quote! {visit_bool}, &visitor_label)
        }
        Some(FieldType::FieldTypeI8) => {
          build_declare_visitor(&quote! {i8}, &quote! {visit_i8}, &visitor_label)
        }
        Some(FieldType::FieldTypeU8) => {
          build_declare_visitor(&quote! {u8}, &quote! {visit_u8}, &visitor_label)
        }
        Some(FieldType::FieldTypeI16) => {
          build_declare_visitor(&quote! {i16}, &quote! {visit_i16}, &visitor_label)
        }
        Some(FieldType::FieldTypeU16) => {
          build_declare_visitor(&quote! {u16}, &quote! {visit_u16}, &visitor_label)
        }
        Some(FieldType::FieldTypeI32) => {
          build_declare_visitor(&quote! {i32}, &quote! {visit_i32}, &visitor_label)
        }
        Some(FieldType::FieldTypeU32) => {
          build_declare_visitor(&quote! {u32}, &quote! {visit_u32}, &visitor_label)
        }
        Some(FieldType::FieldTypeI64) => {
          build_declare_visitor(&quote! {i64}, &quote! {visit_i64}, &visitor_label)
        }
        Some(FieldType::FieldTypeU64) => {
          build_declare_visitor(&quote! {u64}, &quote! {visit_u64}, &visitor_label)
        }
        Some(FieldType::FieldTypeF32) => {
          build_declare_visitor(&quote! {f32}, &quote! {visit_f32}, &visitor_label)
        }
        Some(FieldType::FieldTypeF64) => {
          build_declare_visitor(&quote! {f64}, &quote! {visit_f64}, &visitor_label)
        }
        Some(FieldType::FieldTypeStruct { struct_name }) => {
          let struct_id: String = struct_name
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();
          let struct_ident = build_visitor_ident(&label_name, Some(&struct_id));

          Some(quote! {
            #[allow(non_snake_case, non_camel_case_types)]
            struct #struct_ident;
            impl<'de> Visitor<'de> for #struct_ident {
              type Value = #struct_name;

              fn visit_str(self, v: &str) -> Result<Self::Value, String> {
                let content = "<".to_string() + #struct_id + ">" + v + "</" + #struct_id + ">";
                let value : Result<#struct_name, String> = yaserde::de::from_str(&content);
                value
              }
            }
          })
        }
        Some(FieldType::FieldTypeOption { data_type }) => {
          let dt = Box::into_raw(data_type);
          match unsafe { dt.as_ref() } {
            Some(&FieldType::FieldTypeString) => {
              build_declare_visitor(&quote! {String}, &quote! {visit_str}, &visitor_label)
            }
            Some(&FieldType::FieldTypeBool) => {
              build_declare_visitor(&quote! {bool}, &quote! {visit_bool}, &visitor_label)
            }
            Some(&FieldType::FieldTypeI8) => {
              build_declare_visitor(&quote! {i8}, &quote! {visit_i8}, &visitor_label)
            }
            Some(&FieldType::FieldTypeU8) => {
              build_declare_visitor(&quote! {u8}, &quote! {visit_u8}, &visitor_label)
            }
            Some(&FieldType::FieldTypeI16) => {
              build_declare_visitor(&quote! {i16}, &quote! {visit_i16}, &visitor_label)
            }
            Some(&FieldType::FieldTypeU16) => {
              build_declare_visitor(&quote! {u16}, &quote! {visit_u16}, &visitor_label)
            }
            Some(&FieldType::FieldTypeI32) => {
              build_declare_visitor(&quote! {i32}, &quote! {visit_i32}, &visitor_label)
            }
            Some(&FieldType::FieldTypeU32) => {
              build_declare_visitor(&quote! {u32}, &quote! {visit_u32}, &visitor_label)
            }
            Some(&FieldType::FieldTypeI64) => {
              build_declare_visitor(&quote! {i64}, &quote! {visit_i64}, &visitor_label)
            }
            Some(&FieldType::FieldTypeU64) => {
              build_declare_visitor(&quote! {u64}, &quote! {visit_u64}, &visitor_label)
            }
            Some(&FieldType::FieldTypeF32) => {
              build_declare_visitor(&quote! {f32}, &quote! {visit_f32}, &visitor_label)
            }
            Some(&FieldType::FieldTypeF64) => {
              build_declare_visitor(&quote! {f64}, &quote! {visit_f64}, &visitor_label)
            }
            Some(&FieldType::FieldTypeStruct { ref struct_name }) => {
              let struct_ident = Ident::new(
                &format!("{}", struct_name.into_token_stream()),
                Span::call_site(),
              );
              Some(quote! {
                #[allow(non_snake_case, non_camel_case_types)]
                struct #visitor_label;
                impl<'de> Visitor<'de> for #visitor_label {
                  type Value = #struct_ident;
                }
              })
            }
            _ => {
              unimplemented!();
            }
          }
        }
        Some(FieldType::FieldTypeVec { data_type }) => {
          let dt = Box::into_raw(data_type);
          match unsafe { dt.as_ref() } {
            Some(&FieldType::FieldTypeString) => {
              build_declare_visitor(&quote! {String}, &quote! {visit_str}, &visitor_label)
            }
            Some(&FieldType::FieldTypeBool) => {
              build_declare_visitor(&quote! {bool}, &quote! {visit_bool}, &visitor_label)
            }
            Some(&FieldType::FieldTypeI8) => {
              build_declare_visitor(&quote! {i8}, &quote! {visit_i8}, &visitor_label)
            }
            Some(&FieldType::FieldTypeU8) => {
              build_declare_visitor(&quote! {u8}, &quote! {visit_u8}, &visitor_label)
            }
            Some(&FieldType::FieldTypeI16) => {
              build_declare_visitor(&quote! {i16}, &quote! {visit_i16}, &visitor_label)
            }
            Some(&FieldType::FieldTypeU16) => {
              build_declare_visitor(&quote! {u16}, &quote! {visit_u16}, &visitor_label)
            }
            Some(&FieldType::FieldTypeI32) => {
              build_declare_visitor(&quote! {i32}, &quote! {visit_i32}, &visitor_label)
            }
            Some(&FieldType::FieldTypeU32) => {
              build_declare_visitor(&quote! {u32}, &quote! {visit_u32}, &visitor_label)
            }
            Some(&FieldType::FieldTypeI64) => {
              build_declare_visitor(&quote! {i64}, &quote! {visit_i64}, &visitor_label)
            }
            Some(&FieldType::FieldTypeU64) => {
              build_declare_visitor(&quote! {u64}, &quote! {visit_u64}, &visitor_label)
            }
            Some(&FieldType::FieldTypeF32) => {
              build_declare_visitor(&quote! {f32}, &quote! {visit_f32}, &visitor_label)
            }
            Some(&FieldType::FieldTypeF64) => {
              build_declare_visitor(&quote! {f64}, &quote! {visit_f64}, &visitor_label)
            }
            Some(&FieldType::FieldTypeStruct { ref struct_name }) => {
              let struct_ident = Ident::new(
                &format!("{}", struct_name.into_token_stream()),
                Span::call_site(),
              );
              Some(quote! {
                #[allow(non_snake_case, non_camel_case_types)]
                struct #visitor_label;
                impl<'de> Visitor<'de> for #visitor_label {
                  type Value = #struct_ident;
                }
              })
            }
            _ => None,
          }
        }
        None => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(TokenStream::new(), |mut sum, val| {
      sum.append_all(val);
      sum
    });

  let call_visitors: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      let label = &field.ident;
      let value_label = &get_value_label(&field.ident);

      if field_attrs.attribute {
        return None;
      }

      let label_name = field_attrs
        .rename
        .clone()
        .unwrap_or_else(|| label.as_ref().unwrap().to_string());

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => {
          let visitor = Ident::new("visit_str", Span::call_site());
          build_call_visitor(
            &quote! {String},
            &visitor,
            &quote! {= value},
            &field_attrs,
            label,
            &namespaces,
          )
        }
        Some(FieldType::FieldTypeBool) => {
          let visitor = Ident::new("visit_bool", Span::call_site());
          build_call_visitor(
            &quote! {bool},
            &visitor,
            &quote! {= value},
            &field_attrs,
            label,
            &namespaces,
          )
        }
        Some(FieldType::FieldTypeI8) => {
          let visitor = Ident::new("visit_i8", Span::call_site());
          build_call_visitor(
            &quote! {i8},
            &visitor,
            &quote! {= value},
            &field_attrs,
            label,
            &namespaces,
          )
        }
        Some(FieldType::FieldTypeU8) => {
          let visitor = Ident::new("visit_u8", Span::call_site());
          build_call_visitor(
            &quote! {u8},
            &visitor,
            &quote! {= value},
            &field_attrs,
            label,
            &namespaces,
          )
        }
        Some(FieldType::FieldTypeU16) => {
          let visitor = Ident::new("visit_u16", Span::call_site());
          build_call_visitor(
            &quote! {u16},
            &visitor,
            &quote! {= value},
            &field_attrs,
            label,
            &namespaces,
          )
        }
        Some(FieldType::FieldTypeI16) => {
          let visitor = Ident::new("visit_i16", Span::call_site());
          build_call_visitor(
            &quote! {i16},
            &visitor,
            &quote! {= value},
            &field_attrs,
            label,
            &namespaces,
          )
        }
        Some(FieldType::FieldTypeU32) => {
          let visitor = Ident::new("visit_u32", Span::call_site());
          build_call_visitor(
            &quote! {u32},
            &visitor,
            &quote! {= value},
            &field_attrs,
            label,
            &namespaces,
          )
        }
        Some(FieldType::FieldTypeI32) => {
          let visitor = Ident::new("visit_i32", Span::call_site());
          build_call_visitor(
            &quote! {i32},
            &visitor,
            &quote! {= value},
            &field_attrs,
            label,
            &namespaces,
          )
        }
        Some(FieldType::FieldTypeU64) => {
          let visitor = Ident::new("visit_u64", Span::call_site());
          build_call_visitor(
            &quote! {u64},
            &visitor,
            &quote! {= value},
            &field_attrs,
            label,
            &namespaces,
          )
        }
        Some(FieldType::FieldTypeI64) => {
          let visitor = Ident::new("visit_i64", Span::call_site());
          build_call_visitor(
            &quote! {i64},
            &visitor,
            &quote! {= value},
            &field_attrs,
            label,
            &namespaces,
          )
        }
        Some(FieldType::FieldTypeF32) => {
          let visitor = Ident::new("visit_f32", Span::call_site());
          build_call_visitor(
            &quote! {f32},
            &visitor,
            &quote! {= value},
            &field_attrs,
            label,
            &namespaces,
          )
        }
        Some(FieldType::FieldTypeF64) => {
          let visitor = Ident::new("visit_f64", Span::call_site());
          build_call_visitor(
            &quote! {f64},
            &visitor,
            &quote! {= value},
            &field_attrs,
            label,
            &namespaces,
          )
        }
        Some(FieldType::FieldTypeStruct { struct_name }) => Some(quote! {
          #label_name => {
            reader.set_map_value();
            match #struct_name::deserialize(reader) {
              Ok(parsed_item) => {
                #value_label = parsed_item;
                let _root = reader.next_event();
              },
              Err(msg) => {
                return Err(msg);
              },
            }
          }
        }),
        Some(FieldType::FieldTypeOption { data_type }) => {
          let dt = Box::into_raw(data_type);
          match unsafe { dt.as_ref() } {
            Some(&FieldType::FieldTypeString) => {
              let visitor = Ident::new("visit_str", Span::call_site());
              build_call_visitor(
                &quote! {String},
                &visitor,
                &quote! {= Some(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeBool) => {
              let visitor = Ident::new("visit_bool", Span::call_site());
              build_call_visitor(
                &quote! {bool},
                &visitor,
                &quote! {= Some(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeU8) => {
              let visitor = Ident::new("visit_u8", Span::call_site());
              build_call_visitor(
                &quote! {u8},
                &visitor,
                &quote! {= Some(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeI8) => {
              let visitor = Ident::new("visit_i8", Span::call_site());
              build_call_visitor(
                &quote! {i8},
                &visitor,
                &quote! {= Some(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeU16) => {
              let visitor = Ident::new("visit_u16", Span::call_site());
              build_call_visitor(
                &quote! {u16},
                &visitor,
                &quote! {= Some(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeI16) => {
              let visitor = Ident::new("visit_i16", Span::call_site());
              build_call_visitor(
                &quote! {i16},
                &visitor,
                &quote! {= Some(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeU32) => {
              let visitor = Ident::new("visit_u32", Span::call_site());
              build_call_visitor(
                &quote! {u32},
                &visitor,
                &quote! {= Some(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeI32) => {
              let visitor = Ident::new("visit_i32", Span::call_site());
              build_call_visitor(
                &quote! {i32},
                &visitor,
                &quote! {= Some(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeU64) => {
              let visitor = Ident::new("visit_u64", Span::call_site());
              build_call_visitor(
                &quote! {u64},
                &visitor,
                &quote! {= Some(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeI64) => {
              let visitor = Ident::new("visit_i64", Span::call_site());
              build_call_visitor(
                &quote! {i64},
                &visitor,
                &quote! {= Some(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeF32) => {
              let visitor = Ident::new("visit_f32", Span::call_site());
              build_call_visitor(
                &quote! {f32},
                &visitor,
                &quote! {= Some(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeF64) => {
              let visitor = Ident::new("visit_f64", Span::call_site());
              build_call_visitor(
                &quote! {f64},
                &visitor,
                &quote! {= Some(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeStruct { ref struct_name }) => {
              let struct_ident = Ident::new(
                &format!("{}", struct_name.into_token_stream()),
                Span::call_site(),
              );
              Some(quote! {
                #label_name => {
                  reader.set_map_value();
                  match #struct_ident::deserialize(reader) {
                    Ok(parsed_item) => {
                      #value_label = Some(parsed_item);
                      let _root = reader.next_event();
                    },
                    Err(msg) => {
                      return Err(msg);
                    },
                  }
                }
              })
            }
            _ => unimplemented!(),
          }
        }
        Some(FieldType::FieldTypeVec { data_type }) => {
          let dt = Box::into_raw(data_type);
          match unsafe { dt.as_ref() } {
            Some(&FieldType::FieldTypeString) => {
              let visitor = Ident::new("visit_str", Span::call_site());
              build_call_visitor(
                &quote! {String},
                &visitor,
                &quote! {.push(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeBool) => {
              let visitor = Ident::new("visit_bool", Span::call_site());
              build_call_visitor(
                &quote! {bool},
                &visitor,
                &quote! {.push(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeI8) => {
              let visitor = Ident::new("visit_i8", Span::call_site());
              build_call_visitor(
                &quote! {i8},
                &visitor,
                &quote! {.push(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeU8) => {
              let visitor = Ident::new("visit_u8", Span::call_site());
              build_call_visitor(
                &quote! {u8},
                &visitor,
                &quote! {.push(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeI16) => {
              let visitor = Ident::new("visit_i16", Span::call_site());
              build_call_visitor(
                &quote! {i16},
                &visitor,
                &quote! {.push(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeU16) => {
              let visitor = Ident::new("visit_u16", Span::call_site());
              build_call_visitor(
                &quote! {u16},
                &visitor,
                &quote! {.push(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeI32) => {
              let visitor = Ident::new("visit_i32", Span::call_site());
              build_call_visitor(
                &quote! {i32},
                &visitor,
                &quote! {.push(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeU32) => {
              let visitor = Ident::new("visit_u32", Span::call_site());
              build_call_visitor(
                &quote! {u32},
                &visitor,
                &quote! {.push(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeI64) => {
              let visitor = Ident::new("visit_i64", Span::call_site());
              build_call_visitor(
                &quote! {i64},
                &visitor,
                &quote! {.push(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeU64) => {
              let visitor = Ident::new("visit_u64", Span::call_site());
              build_call_visitor(
                &quote! {u64},
                &visitor,
                &quote! {.push(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeF32) => {
              let visitor = Ident::new("visit_f32", Span::call_site());
              build_call_visitor(
                &quote! {f32},
                &visitor,
                &quote! {.push(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeF64) => {
              let visitor = Ident::new("visit_f64", Span::call_site());
              build_call_visitor(
                &quote! {f64},
                &visitor,
                &quote! {.push(value)},
                &field_attrs,
                label,
                &namespaces,
              )
            }
            Some(&FieldType::FieldTypeStruct { ref struct_name }) => {
              let struct_ident = Ident::new(
                &format!("{}", struct_name.into_token_stream()),
                Span::call_site(),
              );
              Some(quote! {
                #label_name => {
                  reader.set_map_value();
                  match #struct_ident::deserialize(reader) {
                    Ok(parsed_item) => {
                      #value_label.push(parsed_item);
                      let _root = reader.next_event();
                    },
                    Err(msg) => {
                      return Err(msg);
                    },
                  }
                }
              })
            }
            _ => unimplemented!(),
          }
        }
        None => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(TokenStream::new(), |mut sum, val| {
      sum.append_all(val);
      sum
    });

  let attributes_loading: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);
      if !field_attrs.attribute {
        return None;
      }

      let label = &get_value_label(&field.ident);

      let label_name = field_attrs
        .rename
        .unwrap_or_else(|| field.ident.as_ref().unwrap().to_string());

      let visitor_label = build_visitor_ident(&label_name, None);

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => Some(quote! {
          for attr in attributes {
            if attr.name.local_name == #label_name {
              #label = attr.value.to_owned();
            }
          }
        }),
        Some(FieldType::FieldTypeBool) => build_call_visitor_for_attribute(
          label,
          &label_name,
          &quote! {= value},
          &quote! {visit_bool},
          &visitor_label,
        ),
        Some(FieldType::FieldTypeI8) => build_call_visitor_for_attribute(
          label,
          &label_name,
          &quote! {= value},
          &quote! {visit_i8},
          &visitor_label,
        ),
        Some(FieldType::FieldTypeU8) => build_call_visitor_for_attribute(
          label,
          &label_name,
          &quote! {= value},
          &quote! {visit_u8},
          &visitor_label,
        ),
        Some(FieldType::FieldTypeI16) => build_call_visitor_for_attribute(
          label,
          &label_name,
          &quote! {= value},
          &quote! {visit_i16},
          &visitor_label,
        ),
        Some(FieldType::FieldTypeU16) => build_call_visitor_for_attribute(
          label,
          &label_name,
          &quote! {= value},
          &quote! {visit_u16},
          &visitor_label,
        ),
        Some(FieldType::FieldTypeI32) => build_call_visitor_for_attribute(
          label,
          &label_name,
          &quote! {= value},
          &quote! {visit_i32},
          &visitor_label,
        ),
        Some(FieldType::FieldTypeU32) => build_call_visitor_for_attribute(
          label,
          &label_name,
          &quote! {= value},
          &quote! {visit_u32},
          &visitor_label,
        ),
        Some(FieldType::FieldTypeI64) => build_call_visitor_for_attribute(
          label,
          &label_name,
          &quote! {= value},
          &quote! {visit_i64},
          &visitor_label,
        ),
        Some(FieldType::FieldTypeU64) => build_call_visitor_for_attribute(
          label,
          &label_name,
          &quote! {= value},
          &quote! {visit_u64},
          &visitor_label,
        ),
        Some(FieldType::FieldTypeF32) => build_call_visitor_for_attribute(
          label,
          &label_name,
          &quote! {= value},
          &quote! {visit_f32},
          &visitor_label,
        ),
        Some(FieldType::FieldTypeF64) => build_call_visitor_for_attribute(
          label,
          &label_name,
          &quote! {= value},
          &quote! {visit_f64},
          &visitor_label,
        ),
        Some(FieldType::FieldTypeOption { data_type }) => {
          let dt = Box::into_raw(data_type);
          match unsafe { dt.as_ref() } {
            Some(&FieldType::FieldTypeString) => build_call_visitor_for_attribute(
              label,
              &label_name,
              &quote! {= Some(value)},
              &quote! {visit_str},
              &visitor_label,
            ),
            Some(&FieldType::FieldTypeBool) => build_call_visitor_for_attribute(
              label,
              &label_name,
              &quote! {= Some(value)},
              &quote! {visit_bool},
              &visitor_label,
            ),
            Some(&FieldType::FieldTypeU8) => build_call_visitor_for_attribute(
              label,
              &label_name,
              &quote! {= Some(value)},
              &quote! {visit_u8},
              &visitor_label,
            ),
            Some(&FieldType::FieldTypeI8) => build_call_visitor_for_attribute(
              label,
              &label_name,
              &quote! {= Some(value)},
              &quote! {visit_i8},
              &visitor_label,
            ),
            Some(&FieldType::FieldTypeU16) => build_call_visitor_for_attribute(
              label,
              &label_name,
              &quote! {= Some(value)},
              &quote! {visit_u16},
              &visitor_label,
            ),
            Some(&FieldType::FieldTypeI16) => build_call_visitor_for_attribute(
              label,
              &label_name,
              &quote! {= Some(value)},
              &quote! {visit_i16},
              &visitor_label,
            ),
            Some(&FieldType::FieldTypeU32) => build_call_visitor_for_attribute(
              label,
              &label_name,
              &quote! {= Some(value)},
              &quote! {visit_u32},
              &visitor_label,
            ),
            Some(&FieldType::FieldTypeI32) => build_call_visitor_for_attribute(
              label,
              &label_name,
              &quote! {= Some(value)},
              &quote! {visit_i32},
              &visitor_label,
            ),
            Some(&FieldType::FieldTypeU64) => build_call_visitor_for_attribute(
              label,
              &label_name,
              &quote! {= Some(value)},
              &quote! {visit_u64},
              &visitor_label,
            ),
            Some(&FieldType::FieldTypeI64) => build_call_visitor_for_attribute(
              label,
              &label_name,
              &quote! {= Some(value)},
              &quote! {visit_i64},
              &visitor_label,
            ),
            Some(&FieldType::FieldTypeF32) => build_call_visitor_for_attribute(
              label,
              &label_name,
              &quote! {= Some(value)},
              &quote! {visit_f32},
              &visitor_label,
            ),
            Some(&FieldType::FieldTypeF64) => build_call_visitor_for_attribute(
              label,
              &label_name,
              &quote! {= Some(value)},
              &quote! {visit_f64},
              &visitor_label,
            ),
            _ => None,
          }
        }
        Some(FieldType::FieldTypeStruct { struct_name }) => {
          let struct_ident = build_visitor_ident(
            &label_name,
            Some(&struct_name.into_token_stream().to_string()),
          );

          Some(quote! {
            for attr in attributes {
              if attr.name.local_name == #label_name {
                let visitor = #struct_ident{};
                match visitor.visit_str(&attr.value) {
                  Ok(value) => {#label = value;}
                  Err(msg) => {return Err(msg);}
                }
              }
            }
          })
        }
        _ => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(TokenStream::new(), |mut sum, val| {
      sum.append_all(val);
      sum
    });

  let set_text: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let label = &get_value_label(&field.ident);
      let field_attrs = YaSerdeAttribute::parse(&field.attrs);

      match get_field_type(field) {
        Some(FieldType::FieldTypeString) => {
          build_set_text_to_value(&field_attrs, label, &quote! {text_content.to_owned()})
        }
        Some(FieldType::FieldTypeBool) => build_set_text_to_value(
          &field_attrs,
          label,
          &quote! {bool::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeI8) => build_set_text_to_value(
          &field_attrs,
          label,
          &quote! {i8::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeU8) => build_set_text_to_value(
          &field_attrs,
          label,
          &quote! {u8::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeI16) => build_set_text_to_value(
          &field_attrs,
          label,
          &quote! {i16::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeU16) => build_set_text_to_value(
          &field_attrs,
          label,
          &quote! {u16::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeI32) => build_set_text_to_value(
          &field_attrs,
          label,
          &quote! {i32::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeU32) => build_set_text_to_value(
          &field_attrs,
          label,
          &quote! {u32::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeI64) => build_set_text_to_value(
          &field_attrs,
          label,
          &quote! {i64::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeU64) => build_set_text_to_value(
          &field_attrs,
          label,
          &quote! {u64::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeF32) => build_set_text_to_value(
          &field_attrs,
          label,
          &quote! {f32::from_str(text_content).unwrap()},
        ),
        Some(FieldType::FieldTypeF64) => build_set_text_to_value(
          &field_attrs,
          label,
          &quote! {f64::from_str(text_content).unwrap()},
        ),

        Some(FieldType::FieldTypeStruct { .. })
        | Some(FieldType::FieldTypeOption { .. })
        | Some(FieldType::FieldTypeVec { .. })
        | None => None,
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(TokenStream::new(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

  let struct_builder: TokenStream = data_struct
    .fields
    .iter()
    .map(|field| {
      let label = &field.ident;
      let value_label = &get_value_label(&field.ident);

      if get_field_type(field).is_some() {
        Some(quote! {
          #label: #value_label,
        })
      } else {
        None
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(TokenStream::new(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

  quote! {
    use xml::reader::XmlEvent;
    use yaserde::Visitor;
    #[allow(unknown_lints, unused_imports)]
    use std::str::FromStr;

    impl YaDeserialize for #name {
      #[allow(unused_variables)]
      fn deserialize<R: Read>(reader: &mut yaserde::de::Deserializer<R>) -> Result<Self, String> {
        let (named_element, struct_namespace) =
          if let XmlEvent::StartElement{name, ..} = reader.peek()?.to_owned() {
            (name.local_name.to_owned(), name.namespace.clone())
          } else {
            (String::from(#root), None)
          };
        debug!("Struct: start to parse {:?}", named_element);

        if let Some(ref namespace) = struct_namespace {
          match namespace.as_str() {
            #namespaces_matches
            bad_ns => {
              let msg = format!("bad namespace for {}, found {}", named_element, bad_ns);
              return Err(msg);
            }
          }
        };

        #variables
        #field_visitors

        loop {
          match reader.peek()?.to_owned() {
            XmlEvent::StartElement{ref name, ref attributes, ..} => {

              match name.local_name.as_str() {
                #call_visitors
                named_element => {
                  let _root = reader.next_event();
                }
                // name => {
                //   return Err(format!("unknown key {}", name))
                // }
              }
              #attributes_loading
            }
            XmlEvent::EndElement{ref name} => {
              if name.local_name == named_element {
                break;
              }
              let _root = reader.next_event();
            }
            XmlEvent::Characters(ref text_content) => {
              #set_text
              let _root = reader.next_event();
            }
            event => {
              return Err(format!("unknown event {:?}", event))
            }
          }
        }

        Ok(#name{#struct_builder})
      }
    }
  }
}

fn build_declare_visitor(
  field_type: &TokenStream,
  visitor: &TokenStream,
  visitor_label: &Ident,
) -> Option<TokenStream> {
  Some(quote! {
    #[allow(non_snake_case, non_camel_case_types)]
    struct #visitor_label;
    impl<'de> Visitor<'de> for #visitor_label {
      type Value = #field_type;

      fn #visitor(self, v: &str) -> Result<Self::Value, String> {
        Ok(#field_type::from_str(v).unwrap())
      }
    }
  })
}

fn build_call_visitor(
  field_type: &TokenStream,
  visitor: &Ident,
  action: &TokenStream,
  field_attrs: &YaSerdeAttribute,
  label: &Option<Ident>,
  namespaces: &BTreeMap<String, String>,
) -> Option<TokenStream> {
  let prefix = field_attrs.prefix.clone();

  // let label = &field.ident;
  let value_label = get_value_label(label);
  let label_name = field_attrs
    .rename
    .clone()
    .unwrap_or_else(|| label.as_ref().unwrap().to_string());

  let visitor_label = build_visitor_ident(&label_name, None);

  let namespaces_matches: TokenStream = namespaces
    .iter()
    .map(|(p, ns)| {
      let str_ns = ns.as_str();
      if prefix == Some(p.to_string()) {
        Some(quote!(#str_ns => {}))
      } else {
        None
      }
    })
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .fold(TokenStream::new(), |mut tokens, token| {
      tokens.append_all(token);
      tokens
    });

  Some(quote! {
    #label_name => {
      let visitor = #visitor_label{};

      if let XmlEvent::StartElement {name, ..} = reader.peek()?.clone() {
        if let Some(namespace) = name.namespace {
          match namespace.as_str() {
            #namespaces_matches
            bad_ns => {
              let msg = format!("bad field namespace for {}, found {}", name.local_name.as_str(), bad_ns);
              return Err(msg);
            }
          }
        }
        reader.set_map_value()
      }

      let result = reader.read_inner_value::<#field_type, _>(|reader| {
        if let XmlEvent::EndElement { .. } = *reader.peek()? {
          return visitor.#visitor("");
        }

        if let Ok(XmlEvent::Characters(s)) = reader.next_event() {
          visitor.#visitor(&s)
        } else {
          Err(format!("unable to parse content for {}", #label_name))
        }
      });

      if let Ok(value) = result {
        #value_label#action
      }
    }
  })
}

fn build_call_visitor_for_attribute(
  label: &Option<Ident>,
  label_name: &str,
  action: &TokenStream,
  visitor: &TokenStream,
  visitor_label: &Ident,
) -> Option<TokenStream> {
  Some(quote! {
    for attr in attributes {
      if attr.name.local_name == #label_name {
        let visitor = #visitor_label{};
        match visitor.#visitor(&attr.value) {
          Ok(value) => {#label #action;}
          Err(msg) => {return Err(msg);}
        }
      }
    }
  })
}

fn build_set_text_to_value(
  field_attrs: &YaSerdeAttribute,
  label: &Option<Ident>,
  action: &TokenStream,
) -> Option<TokenStream> {
  if field_attrs.text {
    Some(quote! {
      #label = #action;
    })
  } else {
    None
  }
}

fn get_value_label(ident: &Option<syn::Ident>) -> Option<syn::Ident> {
  match &ident {
    Some(ident) => Some(syn::Ident::new(
      &format!("__{}_value", ident.to_string()),
      ident.span(),
    )),
    None => None,
  }
}

fn build_visitor_ident(label: &str, struct_id: Option<&str>) -> Ident {
  Ident::new(
    &format!(
      "__Visitor_{}_{}",
      label.replace(".", "_"),
      struct_id.unwrap_or("")
    ),
    Span::call_site(),
  )
}
