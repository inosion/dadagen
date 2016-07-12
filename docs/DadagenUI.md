### DadaGen UI

This has been written to support the simple case of just needing a file for some situation.

Download it from Bintray

[Dadagen UI 0.2.9a](https://bintray.com/artifact/download/inosion/maven/org/inosion/dadagen/dadagen-ui/0.2.9a/dadagen-ui-0.2.9a.jar)


1. Launch the DadagenUI (the jar is "Double Clickable")
  * Double Click
  * java -jar dadagen-ui-0.2.9.jar
2. Copy in your confguration 
3. Specify How many Rows you want
4. Specify the Output Filename
5. Choose your file type (CSV, JSON, XML)
6. Click Generate and DadagenUI will get to work.

#### Screenshots

This is a Screenshot of the Dadagen UI.


![Dadagen UI Screenshot] (../assets/Dadagen-UI-ScreenShot_v0.2.9.png)


#### Features

* Outputs to CSV, JSON and XML File Formats using Jackson
* Uses RSyntaxArea Editor with the Scala Language
* The Configuration in the Editor _is_ Scala. Therefore, with some magic thinking you can enhance your randomness
* Running with java -jar ... you will be able to alter the classpath, adding in your own generators.
* On startup, it selects a Random File Name to save as; Each iteration will prepend a incrementing number.


