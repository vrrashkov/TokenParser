# Token Parser

<a href="https://crates.io/crates/design_token_parser" rel="nofollow"><img alt="Crates.io" src="https://img.shields.io/crates/v/design_token_parser?color=FC8D62&style=flat-square"></a>

## Overview

Token Parser is a universal tool for generating runnable code for any language from your Figma Tokens. It is written in Rust so you have the freedom to use it anywhere you would like without having **node.js** or anything else installed other than the executable on your system. The full configuration is happening through a **configuration yaml** file from which you can customize to build for as many different languages as you want from a single place. 

## Tested with

1. [Figma Variables](https://www.figma.com/plugin-docs/working-with-variables/)

2. [Figma Studio Tokens](https://github.com/tokens-studio/figma-plugin)

3. [lukasoppermann/design-tokens](https://github.com/lukasoppermann/design-tokens)

> You can use tokens from multiple sources as long as they are with the correct json structure!

## Setup

You can get the whole project and build it yourself or if you don't have Rust or just don't want to deal with the builds yourself, go in the Release section and get the executables from there. 

1. Setup the **`assts/design_tokens_config.yaml`** file
2. Run with: for windows (`WIN_design_token_parser.exe`) for MAC (`MAC_design_token_parser`) you can find them in Release section

```shell
   .\WIN_design_token_parser.exe --generate --config "path/design_tokens_config.yaml"
```

That's all, your files will be generated and ready to use

The process for generating the usable tokens is split into two.

- Converting the Figma tokens to usable json files (similar to **style-dictionary**)

- Generating the end files for the langages from the previously generated json files

If you have already generated the usable json files you can just run the end code generation by running.

```shell
 WIN_design_token_parser.exe --config "path/design_tokens_config.yaml"
```

### Homebrew

- brew tap vrrashkov/tokenparser
- brew install tokenparser
- Configure the yaml config file
- `design_token_parser --generate --config "design_tokens_config.yaml"`

## Configuration

##### Input/Output paths for loading and generation

```yaml
global: 
  # Figma source paths
  # These are the pure files from Figma, they can contain aliases
  # For example if we have aliases we will need the actual value and not the alias
  # Separating different files is necessary in case there are duplicate trees but different values/aliases
  # So if we have button-md and button-big with the same trees but different values with aliases that need to be accesed from core.json
  # this should be the setup
  # Look at the figma/variables and figma/generated_styles for better understanding how it works
  figma_source_paths: 
    - combine:
        file_name: "button-lg"
        files:
          - "assets/figma/variables/button-lg.json"
          - "assets/figma/variables/core-.json"
    - combine:
        file_name: "button-md"
        files:
          - "assets/figma/variables/button-md.json"
          - "assets/figma/variables/core-.json"
    - combine:
        file_name: "button-sm"
        files:
          - "assets/figma/variables/button-sm.json"
          - "assets/figma/variables/core-.json"
    - combine:
        file_name: "color-light"
        files:
          - "assets/figma/variables/color-accent-primary.json"
          - "assets/figma/variables/color-accent-secondary.json"
          - "assets/figma/variables/color-status-success.json"
          - "assets/figma/variables/color-status-danger.json"
          - "assets/figma/variables/color-light.json"
          - "assets/figma/variables/palette-.json"
    - combine:
        file_name: "color-dark"
        files:
          - "assets/figma/variables/color-accent-primary.json"
          - "assets/figma/variables/color-accent-secondary.json"
          - "assets/figma/variables/color-status-success.json"
          - "assets/figma/variables/color-status-danger.json"
          - "assets/figma/variables/color-dark.json"
          - "assets/figma/variables/palette-.json"
  # file_name: If set this will be the name of the merged file
  # if not, than the first file name will be used
  figma_output_paths:
    - combine:
        file_name: "button-lg"
        files:
          - path: "assets/figma/variables/button-lg.json"
    - combine:
        file_name: "button-md"
        files:
          - path: "assets/figma/variables/button-md.json"
    - combine:
        file_name: "button-sm"
        files:
          - path: "assets/figma/variables/button-sm.json"
    - combine:
        file_name: "color-light"
        files:
          - path: "assets/figma/variables/color-accent-primary.json"
            # if mode is set this will wrap the whole json object with a parent of the mode's value
            # this helps if you have similar trees in multiple files
            mode: "primary"
          - path: "assets/figma/variables/color-accent-secondary.json"
            mode: "secondary"
          - path: "assets/figma/variables/color-status-success.json"
            mode: "success"
          - path: "assets/figma/variables/color-status-danger.json"
            mode: "danger"
          - path: "assets/figma/variables/color-light.json"
    - combine:
        file_name: "color-dark"
        files:
          - path: "assets/figma/variables/color-accent-primary.json"
            mode: "primary"
          - path: "assets/figma/variables/color-accent-secondary.json"
            mode: "secondary"
          - path: "assets/figma/variables/color-status-success.json"
            mode: "success"
          - path: "assets/figma/variables/color-status-danger.json"
            mode: "danger"
          - path: "assets/figma/variables/color-dark.json"
  #Output path 
  style_output_path: "assets/generated_styles/"
```

##### Template config

```yaml
templates:
  - settings_general:
      generate_file_path: "generated_templates"
      file_name:
        format: "DS{{style}}"
        extension: "swift"
        #case: "kebab"
    settings_custom:
      # For header and footer {{style}} is a secial variable that can be used
      header: 
        - "import SwiftUI"
        - "public class DSCore{{style}} {"
      footer: 
        - "}"
      template_type:
        # For themes
        - type: color
          value: "public static let {{variable_name | camel}} = {{value | color: 'Color(red: rgb_r_v1, green: rgb_g_v1, blue: rgb_b_v1, opacity: rgb_a_v1)'}}  {{description | optional: '// desc = %value'}}"
        # For Core
        - type: string
          value: "public static let {{variable_name | camel}} = {{value}}  {{description | optional: '// desc = %value'}}"   
        - type: float
          value: "public static let {{variable_name | camel}} = CGFloat({{value | as_text_or_number}})  {{description | optional: '// desc = %value'}}"   
        - type: boolean
          value: "public static let {{variable_name | camel}} = {{value}}  {{description | optional: '// des        c = %value'}}"   
        - type: composition
          value: "{% if verticalPadding != '' %} test1: {{verticalPadding | optional: 'vertical-padding-test-first: %value'}} {% endif %}"
        - type: composition
          value: "{% if verticalPadding != '' %} test2: {{verticalPadding | optional: 'vartical-padding-test-second: %value'}} {% endif %}"
        - type: boxShadow
          value: 
            - "{{variable_name}} {{color-0 | color: 'hex'}} blur: {{blur-0}} x: {{x-0}}"
            - "{{variable_name}} {{color-0 | color: 'hex'}} {{color-1 | color: 'hex'}}  blur: {{blur-0}} x: {{x-0}} blur: {{blur-1}} x: {{x-1}}"
```

You can use every type multiple times for more clean way of creating your values. There are many **filters** that can help you create the template you want (check them bellow). Also because this tool is using [Liquid](https://github.com/cobalt-org/liquid-rust) you can expect every filter/tags/blocks to be usable in your templates. As you can see from the above code there are if statements that check if a variable is present and if it is display something. 

Optional values can be added with the **optional** filter. Instead of using if statements sometimes it's easier to just use the **optional** filter and display the value only if it exists.

##### Valid JSON

Both type of jsons are valid and represent the same structure becase of the forward slash, you can have infinite amount of nesting or no nesting at all.  

```json
{
    "size/XL": {
        "type": "float",
        "value": "56",
        "description": ""
    },
    "text/fr": {
        "type": "string",
        "value": "Some Text",
        "description": ""
    },
    "color/bg": {
        "type": "color",
        "value": "#000000",
        "description": ""
    }
}
```

```json
{
    "size": {
        "XL": {
            "type": "float",
            "value": "56",
            "description": ""
        }
    },
    "text": {
        "fr": {
            "type": "string",
            "value": "Some Text",
            "description": ""
        }
    },
    "color": {
        "bg": {
            "type": "color",
            "value": "#000000",
            "description": ""
        }
    }
}
```

##### JSON value format

> You can have as much nesting of the tree as you want as long as the end node contains `value`and `type`(`description`is optional) 

###### Valid Examples and how to use JSON -> Template

-----------------------------------------

```json
   "XL": {
        "type": "float",
        "value": "56",
        "description": ""
    }
```

```yaml
- type: float
  value: "public static let {{variable_name | camel}} = CGFloat({{value | as_text_or_number}})  {{description | optional: '// desc = %value'}}"   
```

```swift
public static let xl = CGFloat(56) 
```

---

```json
"bold": {
   "type": "typography",
   "value": {
      "fontFamily": "Noir Pro",
      "fontSize": "16",
      "fontWeight": "Bold",
      "letterSpacing": "-0.41",
      "lineHeight": "23"
    }
}
```

```yaml
- type: typography
  value: "public static let {{variable_name | camel}} = TextStyle(name: \"{{fontFamily}}\", size: {{fontSize}}, weight: TextStyle.Weight({{fontWeight | as_text_or_number}}), lineHeight: {{lineHeight}})"
```

```swift
public static let bold = TextStyle(name: "Noir Pro", size: 16, weight: TextStyle.Weight("Bold"), lineHeight: 23)
```

---

```json
    "tabBar": {
      "type": "boxShadow",
      "value": [
        {
          "blur": "20",
          "color": "rgba(0,0,0,0.1)",
          "spread": "0",
          "type": "dropShadow",
          "x": "0",
          "y": "0"
        },
        {
          "blur": "8",
          "color": "rgba(0,0,0,0.1)",
          "spread": "0",
          "type": "dropShadow",
          "x": "0",
          "y": "4"
        }
      ]
    }
```

```yaml
- type: boxShadow
  value: 
    - "public static let {{variable_name | camel}} = Shadow(x: CGFloat({{x-0}}), y: CGFloat({{y-0}}), color: Color(hex: \"{{color-0 | color: 'hex'}}\"), radius: CGFloat({{blur-0}}))"
    - "public static let {{variable_name | camel}} = [Shadow(x: CGFloat({{x-0}}), y: CGFloat({{y-0}}), color: Color(hex: \"{{color-0 | color: 'hex'}}\"), radius: CGFloat({{blur-0}})), Shadow(x: CGFloat({{x-1}}), y: CGFloat({{y-1}}), color: Color(hex: \"{{color-1 | color: 'hex'}}\"), radius: CGFloat({{blur-1}}))]"
      
```

```swift
public static let shadowTabBar = [Shadow(x: CGFloat(0), y: CGFloat(0), color: Color(hex: "#00000019"), radius: CGFloat(20)), Shadow(x: CGFloat(0), y: CGFloat(4), color: Color(hex: "#00000019"), radius: CGFloat(8))]
```

---

```json
"dissolve": {
  "description": null,
  "type": "custom-transition",
  "value": {
    "duration": 0.45,
    "easingFunction": {
      "x1": 0.6968395709991455,
      "x2": 0.06683959811925888,
      "y1": 0.05232666060328483,
      "y2": 0.9323266744613647
    },
    "transitionType": "dissolve"
  }
}
```

```yaml
# Object
# {{easingFunction | empty}} use empty filter to initialize variable if you need it in the scope of the template without printing it
- type: "custom-transition"
  value: "public static let {{variable_name | camel}} = CustomTransition(duration: {{duration}},
    {{easingFunction | empty}}
    x1: CGFloat({{easingFunction.x1}}),
    x2: CGFloat({{easingFunction.x2}}),
    y1: CGFloat({{easingFunction.y1}}),
    y2: CGFloat({{easingFunction.y2}}))
  " 
```

```swift
public static let dissolve = CustomTransition(duration: 0.45,  x1: CGFloat(0.6968395709991455), x2: CGFloat(0.06683959811925888), y1: CGFloat(0.05232666060328483), y2: CGFloat(0.9323266744613647)) 
```
---

```json
"gradient": {
  "single with multiple color stops": {
    "description": "Four color stops from yellow to red",
    "type": "custom-gradient",
    "value": {
      "gradientType": "radial",
      "rotation": 180,
      "stops": [
        {
          "color": "#ffb800ff",
          "position": 0
        },
        {
          "color": "#ff8a00ff",
          "position": 0.34
        },
        {
          "color": "#ff2e00ff",
          "position": 0.65
        },
        {
          "color": "#ff0000ff",
          "position": 1
        }
      ]
    }
  }
}
```

```yaml
# Array
# {{stops | empty}} use empty filter to initialize variable if you need it in the scope of the template without printing it
- type: "custom-gradient"
  value: "public static let {{variable_name | camel}} = CustomGradient(gradientType: {{gradientType}}, rotation: {{rotation}},
  {{stops | empty}}

  color1: Style1(
    {% for stop in stops %}

    {{stop.color | color: 'Color(red: rgb_r_v1, green: rgb_g_v1, blue: rgb_b_v1, opacity: rgb_a_v1)'}}
    {% if forloop.last == false -%}, {% endif %}
    {% endfor -%}
  ),

  color2: Style2(
    {% assign colors = stops | map: 'color' %}
    {% for item in colors %}
    
    {{item | color: 'Color(red: rgb_r_v2, green: rgb_g_v2, blue: rgb_b_v2, opacity: rgb_a_v2)'}}
    {% if forloop.last == false -%}, {% endif %}
    {% endfor %}
  )
  " 
```

```swift
public static let defaultGradientSingleWithMultipleColorStops = CustomGradient(gradientType: radial, rotation: 180, 
color1: Style1( 
Color(red: 1.000, green: 0.722, blue: 0.000, opacity: 1.000) ,  
Color(red: 1.000, green: 0.541, blue: 0.000, opacity: 1.000) ,  
Color(red: 1.000, green: 0.180, blue: 0.000, opacity: 1.000) ,  
Color(red: 1.000, green: 0.000, blue: 0.000, opacity: 1.000)  ),
color2: Style2(  
Color(red: 255, green: 184, blue: 0, opacity: 184) ,  
Color(red: 255, green: 138, blue: 0, opacity: 138) ,  
Color(red: 255, green: 46, blue: 0, opacity: 46) ,  
Color(red: 255, green: 0, blue: 0, opacity: 0)   ) 
}
```

---

> Use `{{easingFunction | empty}}` empty filter to initialize a variable without displaying it. This filter is unique to this library and necessary for a lot of the blocks that the liquid templating provides.

##### Keywords

You can use the keywords in the following way: **`{{variable_name | kebab }}`** name of the keyword and next to it you can add a filter, or multiple filters by separating them with **`|`** like this **`{{variable_name | kebab | no_space }}`**.

##### Special filters

| name                                 | extra options/info                                                                                                                                                         |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `empty`                              | Initialize variable without displaying it's value                                                                                                                          |
| `no_space`                           | If the value contains space, remove it. For example "`Test No Space`" will turnto "`TestNoSpace`"                                                                          |
| `as_text_or_number`                  | If the value is text it will add double quotes to it.                                                                                                                      |
| `pascal`, <br/>`kebab`, <br/>`camel` | Different case filters                                                                                                                                                     |
| `color`                              | `rgb_r_v1`, `rgb_g_v1`, `rgb_b_v1`, `rgb_a_v1`<br/>`rgb_r_v2`, `rgb_g_v2`, `rgb_b_v2`, `rgb_a_v2`<br/>`hex`<br/><br/>v1 - values from 0 to 255<br/>v2 - values from 0 to 1 |

> You can get example of full configuration from the assets folder

Arrays have a bit different take on how you shuld template them. For example in the file the `boxShadow` type is working like this. Because we can expect array values with uknown length here is how you can handle them.

```yaml
# All the color related values from above
# For every new line of the boxShadow value, a new index can be used. For example:
# On line 1 you have only values with index 0 
# On line 2 you have values with index 0 and 1
# On line 3 you have values with index 0, 1 and 2 and etc.. 
# All possible variants should be made with a template
# If there is a missing one you will be notified with an error to add it
- type: boxShadow
  value: 
    - "{{variable_name}} {{color-0 | color: 'hex'}} blur: {{blur-0}} x: {{x-0}}"
    - "{{variable_name}} {{color-0 | color: 'hex'}} {{color-1 | color: 'hex'}}  blur: {{blur-0}} x: {{x-0}} blur: {{blur-1}} x: {{x-1}}"
```

##### Liquid filters

Check the available filters/blocks and etc.. for the liquid templating from here:

- [Liquid for Designers · Shopify/liquid Wiki · GitHub](https://github.com/Shopify/liquid/wiki/Liquid-for-Designers)

- https://jg-rp.github.io/liquid/language/filters

##### More

For any ideas or issues please don't hasitate to ask/report. 
