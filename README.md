# Token Parser

<a href="https://crates.io/crates/design_token_parser" rel="nofollow"><img alt="Crates.io" src="https://img.shields.io/crates/v/design_token_parser?color=FC8D62&style=flat-square"></a>

## Overview

Token Parser is a tool for generating runnable code for any language from your [Figma Variables](https://www.figma.com/plugin-docs/working-with-variables/) or [Figma Studio Tokens](https://github.com/tokens-studio/figma-plugin) (*you can even use both at the same time*). It is written in Rust so you have the freedom to use it anywhere you would like without having **node.js** or anything else installed other than the executable on your system. The full configuration is happening through a **configuration yaml** file from which you can customize to build for as many different languages as you want from a single place. 

## Setup

You can get the whole project and build it yourself or if you don't have Rust or just don't want to deal with the builds yourself, go in the Release section and get the executables from there. 

1. Setup the **assts/design_tokens_config.yaml** file
2. Run with: for windows (WIN_design_token_parser.exe) for MAC (MAC_design_token_parser) you can find them in Release section

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
- design_token_parser --generate --config "design_tokens_config.yaml"
  
## Configuration

##### Input/Output paths for loading and generation

```yaml
global: 
  # Figma variables source paths
  # These are the pure files from Figma, they can contain aliases
  # For example if we have aliases we will need the actual value and not the alias
  # Separating different files is necessary in case there are duplicate trees but different values/aliases
  # So if we have button-md and button-big with the same trees but different values with aliases that need to be accesed from core.json
  # this should be the setup
  # Look at the figma/variables and figma/generated_styles for better understanding how it works
  figma_variables_source_paths: 
    - combine:
        files:
          - "assets/figma/variables/color-light.json"
          - "assets/figma/variables/color-dark.json"
    - combine:
        files:
        - "assets/figma/variables/button-md.json"
        - "assets/figma/variables/core.json"
    - combine:
        files:
        - "assets/figma/variables/button-big.json"
        - "assets/figma/variables/core.json"
  # Figma studio source paths
  # figma_studio_source_paths: 
  #   - "assets/figma/studio/core.json"
  #   - "assets/figma/studio/dark.json"
  #   - "assets/figma/studio/global.json"
  #   - "assets/figma/studio/light.json"
  #   - "assets/figma/studio/mobile.json"
  #   - "assets/figma/studio/m-button.json"
  #   - "assets/figma/studio/m-progress.json"
  #   - "assets/figma/studio/m-reg.json"
  #   - "assets/figma/studio/typography.json"
  # Figma output calculated files, 
  # file_name: If set this will be the name of the merged file
  # if not, than the first file name will be used
  figma_output_paths:
    - combine:
        file_name: "color-light"
        files:
          - "assets/figma/variables/color-light.json"
    - combine:
        file_name: "color-dark"
        files:
          - "assets/figma/variables/color-dark.json"
    - combine:
        file_name: "button-md"
        files:
          - "assets/figma/variables/button-md.json"
    - combine:
        file_name: "button-big"
        files:
          - "assets/figma/variables/button-big.json"
  # Different generated
  output_paths:
    - combine:
        files:
          - "color-light"
    - combine:
        files:
          - "color-dark"
    - combine:
        files:
          - "button-md"
    - combine:
        files:
          - "button-big"
  #Output path 
  style_output_path: "assets/generated_styles/"
```

##### Template config

```yaml
templates:
  - settings_general:
      generate_file_path: "generated_templates"
      file_name:
        format: "DS{style}"
        extension: "swift"
        #case: "kebab"
        #This will replace the class_name from template_type as well
        #use_as_class_name: true
    settings_custom:
      header: "import SwiftUI"
      #sub_header: "test sub header"
      #sub_footer: "test sub footer"
      #footer: "}"
      # Only if class is set, class_name will be displayed
      class: "public class"
      class_name: "DSCore{style}"
      template_type:
        # For themes
        - type: color
          value: "public static let {{variable_name | camel}} = {{color | color: 'Color(red: rgb_r_v1, green: rgb_g_v1, blue: rgb_b_v1, opacity: rgb_a_v1)'}}"
        # For Core
        - type: string
          value: "public static let {{variable_name | camel}} = {{string}}"   
        - type: float
          value: "public static let {{variable_name | camel}} = CGFloat({{float}})"   
        - type: boolean
          value: "public static let {{variable_name | camel}} = {{boolean}}"   
        - type: composition
          value: "{% if verticalPadding != '' %} test1: {{verticalPadding | optional: 'vertical-padding-test-first: %value'}} {% endif %}"
        - type: composition
          value: "{% if verticalPadding != '' %} test2: {{verticalPadding | optional: 'vartical-padding-test-second: %value'}} {% endif %}"
        - type: boxShadow
          value: 
            - "{{variable_name}} {{color-0 | color: 'hex'}} blur: {{blur-0}} x: {{x-0}}"
            - "{{variable_name}} {{color-0 | color: 'hex'}} {{color-1 | color: 'hex'}}  blur: {{blur-0}} x: {{x-0}} blur: {{blur-1}} x: {{x-1}}"
```

In the above scenario 4 files are going to be generated **DSButtonBig**, **DSButtonMd**, **DSColorDark** and **DSColorLight** they will be containing tokens which we set through **template_type** (**ONLY** if those tokens are available) in this case **composition**, **boxShadow**, **string**, **float**, **boolean** and **color**, in this case we are using Figma Variables but if we want we can have have also tokens from Figma Studio and use them together. <u>Every different type contains specific keys you can use to create the template that you want</u>. See below the list of special keywords you can use.

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

##### Keywords

| type             | value                                                                                                                                                                                                                                                                                        |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| GLOBAL VALUES    | variable_name<br/>description                                                                                                                                                                                                                                                                |
| color            | color                                                                                                                                                                                                                                                                                        |
| float            | float                                                                                                                                                                                                                                                                                        |
| string           | string                                                                                                                                                                                                                                                                                       |
| boolean          | boolean                                                                                                                                                                                                                                                                                      |
| typography       | fontFamily<br/>fontSize<br/>fontWeight<br/>spacing<br/>lineHeight<br/>paragraphSpacing<br/>paragraphIndent<br/>textCase<br/>textDecoration                                                                                                                                                   |
| paragraphSpacing | paragraphSpacing                                                                                                                                                                                                                                                                             |
| paragraphIndent  | paragraphIndent                                                                                                                                                                                                                                                                              |
| textCase         | textCase                                                                                                                                                                                                                                                                                     |
| textDecoration   | textDecoration                                                                                                                                                                                                                                                                               |
| spacing          | spacing                                                                                                                                                                                                                                                                                      |
| borderWidth      | borderWidth                                                                                                                                                                                                                                                                                  |
| borderRadius     | borderRadius                                                                                                                                                                                                                                                                                 |
| letterSpacing    | spacing                                                                                                                                                                                                                                                                                      |
| lineHeights      | lineHeight                                                                                                                                                                                                                                                                                   |
| fontSizes        | fontSize                                                                                                                                                                                                                                                                                     |
| fontWeights      | fontWeight                                                                                                                                                                                                                                                                                   |
| fontFamilies     | fontFamily                                                                                                                                                                                                                                                                                   |
| sizing           | sizing                                                                                                                                                                                                                                                                                       |
| other            | other                                                                                                                                                                                                                                                                                        |
| boxShadow        | color<br/>blur<br/>spread<br/>type<br/>x<br/>y                                                                                                                                                                                                                                               |
| composition      | paddingBottom<br/>paddingTop<br/>paddingLeft<br/>paddingRight<br/>sizing<br/>height<br/>width<br/>borderRdius<br/>borderWidth<br/>borderRadiusBottomLeft<br/>border_radiusBottomRight<br/>borderRadiusTopLeft<br/>borderRadiusTopRight<br/>spacing<br/>verticalPadding<br/>horizontalPadding |

You can use the keywords in the following way: **{{variable_name | kebab }}** name of the keyword and next to it you can add a filter, or multiple filters by separating them with **|** lie this **{{variable_name | kebab | no_space }}**.

##### Variants

| type            | variant                                                                                                                                                  |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------- |
| added to GLOBAL | no_space, pascal, kebab, camel                                                                                                                           |
| color           | rgb_r_v1, rgb_g_v1, rgb_b_v1, rgb_a_v1<br/>rgb_r_v2, rgb_g_v2, rgb_b_v2, rgb_a_v2<br/>hex<br/><br/>v1 - values from 0 to 255<br/>v2 - values from 0 to 1 |

> You can get example of full configuration from the assets folder

**boxShadow** has a bit different take on how you shuld template it. Because we can expect array values with uknown length here is how you can handle them.

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

##### More

For any ideas or issues please don't hasitate to ask/report. 
