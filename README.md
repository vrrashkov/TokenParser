# Token Parser

## Overview

Token Parser is a tool for generating runnable code for any language from your [Figma tokens](https://github.com/tokens-studio/figma-plugin). It is written in Rust so you have the freedom to use it anywhere you would like without having **node.js** or anything else installed other than the executable on your system. The full configuration is happening through a **configuration yaml** file from which you can customize to build for as many different languages as you want from a single place. 

## Setup

You can get the whole project and build it yourself or if you don't have Rust or just don't want to deal with the builds yourself, go in the Release section and get the executables from there. 

1. Setup the **configuration.yaml** file

2. Run with 

```shell
   "executable" --generate --config "path/design_tokens_config.yaml"
```

That's all, your files will be generated and ready to use

The process for generating the usable tokens is split into two.

- Converting the Figma tokens to usable json files (similar to **tokenizer**)

- Generating the end files for the langages from the previously generated json files

If you have already generated the usable json files you can just run the end code generation by running.

```shell
 "executable" --config "path/design_tokens_config.yaml"
```

## Configuration

##### Input/Output paths for loading and generation

```yaml
global:
  # Other files will be merged togehter
  other_path: 
    - value: 
      - "assets/figma/core.json"
      - "assets/figma/typography.json"
      - "assets/figma/global.json"
    - value: 
      - "assets/figma/mobile.json"
  # Styles will be kept separate
  style_path:
    - "assets/figma/dark.json"
    - "assets/figma/light.json"
  # Output path 
  style_output_path: "assets/generated_styles"
```

The configuration above will end up generating 4 **json** files with usable tokens inside **assets/generated_styles**. 

- mobile.json/core.json
  
  - You can provide all the other files **other_path**. Every separate **value** array will merge into a single file with the first **value** of the array as it's name

- dark.json
  
  - Only the specific tokens for the **dark style**

- light.json
  
  - Only the specific tokens for the **light style**

##### Template config

```yaml
templates:
  - settings_general:
      # Path where the template should be created
      generate_file_path: "generated_templates"
      file_name:
        # Special keyword {style}
        # it will be replaced with the specific style
        # In this case dark/light
        format: "cds-{style}"
        extension: "css"
        case: "kebab"
    settings_custom:
      header: ":root {"
      #sub_header: "test sub header"
      #sub_footer: "test sub footer"
      footer: "}"
      # Only if class is set, class_name will be displayed
      #class: "public class"
      #class_name: "SomeNameCore{style}"
      template_type:
        - type: color
          value: "{{variable_name | kebab}} {{color | color: 'rgb_r_v1, rgb_g_v2'}}"
        - type: composition
          value: "{% if verticalPadding != '' %} test1: {{verticalPadding | optional: 'vertical-padding-test-first: %value'}} {% endif %}"
        - type: composition
          value: "{% if verticalPadding != '' %} test2: {{verticalPadding | optional: 'vartical-padding-test-second: %value'}} {% endif %}"
        - type: boxShadow
          value: 
            - "{{variable_name}} {{color-0 | color: 'hex'}} blur: {{blur-0}} x: {{x-0}}"
            - "{{variable_name}} {{color-0 | color: 'hex'}} {{color-1 | color: 'hex'}}  blur: {{blur-0}} x: {{x-0}} blur: {{blur-1}} x: {{x-1}}"
```

In the above scenario 2 files are going to be generated **cds-dark** and **cds-light** they will both be containing tokens which we set through **template_type** in this case **composition**, **boxShadow** and **color**.  <u>Every different type contains specific keys you can use to create the template that you want</u>. See below the list of special keywords you can use.

You can use every time multiple times for more clean way of creating your values. There are many **filters** that can help you create the template you want (check them bellow). Also because this tool is using [Liquid](https://github.com/cobalt-org/liquid-rust) you can expect every filter/tags/blocks to be usable in your templates. As you can see from the above code there are if statements that check if a variable is present and if it is display something. 

Optional values can be added with the **optional** filter. Instead of using if statements sometimes it's easier to just use the **optional** filter and display the value only if it exists.

##### Keywords

| type          | value                                                                                                                                                                                                                                                                                                                               |
| ------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| color         | variable_name<br/>color                                                                                                                                                                                                                                                                                                             |
| typography    | variable_name<br/>font_family<br/>font_size<br/>font_weight<br/>spacing<br/>line_height                                                                                                                                                                                                                                             |
| spacing       | variable_name<br/>spacing                                                                                                                                                                                                                                                                                                           |
| borderWidth   | variable_name<br/>border_width                                                                                                                                                                                                                                                                                                      |
| borderRadius  | variable_name<br/>border_radius                                                                                                                                                                                                                                                                                                     |
| letterSpacing | variable_name<br/>spacing                                                                                                                                                                                                                                                                                                           |
| lineHeights   | variable_name<br/>line_height                                                                                                                                                                                                                                                                                                       |
| fontSizes     | variable_name<br/>font_size                                                                                                                                                                                                                                                                                                         |
| fontWeights   | variable_name<br/>font_weight                                                                                                                                                                                                                                                                                                       |
| fontFamilies  | variable_name<br/>font_family                                                                                                                                                                                                                                                                                                       |
| boxShadow     | variable_name<br/>color<br/>blur<br/>spread<br/>type<br/>x<br/>y                                                                                                                                                                                                                                                                    |
| composition   | variable_name<br/>padding_bottom<br/>padding_top<br/>padding_left<br/>padding_right<br/>sizing<br/>height<br/>width<br/>border_radius<br/>border_width<br/>border_radius_bottom_left<br/>border_radius_bottom_right<br/>border_radius_top__left<br/>border_radius_top_right<br/>spacing<br/>vertical_padding<br/>horizontal_padding |

You can use the keywords in the following way: **{variable_name | kebab }** name of the keyword and next to it you can add a filter, or multiple filters by separating them with **|** lie this **{variable_name | kebab | no_space }**.

##### Variants

| type            | variant                                                                                                                                                                                                                                          |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| added to GLOBAL | no_space, pascal, kebab, camel                                                                                                                                                                                                                   |
| color           | rgb_r_v1, rgb_g_v1, rgb_b_v1, rgb_a_v1<br/>rgb_r_v2, rgb_g_v2, rgb_b_v2, rgb_a_v2<br/>hex<br/><br/>v1 - values from 0 to 255<br/>v2 - values from 0 to 1                                                                                      \| |

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