import sbt._

/**
 * Our Project Dependencies
 */
object Dependencies {

  import BaseSettings.Version

  def compile   (deps: ModuleID*): Seq[ModuleID] = deps map (_ % "compile")
  def provided  (deps: ModuleID*): Seq[ModuleID] = deps map (_ % "provided")
  def test      (deps: ModuleID*): Seq[ModuleID] = deps map (_ % "test")
  def runtime   (deps: ModuleID*): Seq[ModuleID] = deps map (_ % "runtime")
  def container (deps: ModuleID*): Seq[ModuleID] = deps map (_ % "container")

  // we use this for the case class copier method
  object scalalang {
    val reflect    = "org.scala-lang"     % "scala-reflect"                  % Version.Scala
    val compiler    = "org.scala-lang"     % "scala-compiler"                 % Version.Scala
  }

  object akka {
    val version = "2.3.11"
    val actor = "com.typesafe.akka" %% "akka-actor" % version
  }

  // for inspecting Beans and the like
  object commons { 
    val beanutils  = "commons-beanutils" % "commons-beanutils" % "1.9.4"
  }

  // for generating regexpression strings
  val generex = "com.github.mifmif" % "generex" % "1.0.2"

  // excellent config support
  val typesafeconfig = "com.typesafe" % "config" % "1.4.1"

  // opencsv wrapper
  // "com.bizo:mighty-csv_2.11:0.2"

  object jackson { 
    val version = "2.12.3"
    val dfcsv = "com.fasterxml.jackson.dataformat" % "jackson-dataformat-csv" % version
    val dfxml = "com.fasterxml.jackson.dataformat" % "jackson-dataformat-xml" % version
    val databind = "com.fasterxml.jackson.core" % "jackson-databind" % version
    val scala = "com.fasterxml.jackson.module" %% "jackson-module-scala" % version
  }

  // needed for indenting the XML output in Dadagen-UI
  val stax2woodstox = "org.codehaus.woodstox" % "woodstox-core-asl" % "4.4.1"

  // This is the Java 8 Version val scalafx = "org.scalafx" %% "scalafx" % "8.0.40-R8"
  val scalafx = "org.scalafx" %% "scalafx" % "16.0.0-R24"

  /*
   * Testing and Test helpful Libs
   */
  object gatling {
    val version = "3.6.0"
    val testFramework = "io.gatling"            % "gatling-test-framework" % version
    val highcharts    = "io.gatling.highcharts" % "gatling-charts-highcharts" % version
  }

  object jmeter {

    val version = "5.4.1"
    val core = ("org.apache.jmeter" % "ApacheJMeter_core" % version)
      // it would be _really_ awesome if I could say - exclude "everything" BUT xyz :-)
      // or even support regexp in the "org" and/or "artifactId"
      .exclude("avalon-framework","*")
      .exclude("com.thoughtworks.xstream","xstream")
      .exclude("commons-math3","*")
      .exclude("commons-pool2","*")
      .exclude("commons-codec","*")
      .exclude("commons-collections","*")
      .exclude("commons-httpclient","*")
      .exclude("commons-io","*")
      .exclude("commons-jexl","*")
      .exclude("commons-logging","*")
      .exclude("commons-math3","*")
      .exclude("commons-net","*")
      .exclude("commons-pool2","*")
      .exclude("excalibur-datasource","*")
      .exclude("excalibur-instrument","*")
      .exclude("excalibur-logger","*")
      .exclude("excalibur-pool","*")
      .exclude("bsf","*")
      .exclude("org.apache.commons","*")
      .exclude("dnsjava","dnsjava")
      .exclude("javax.mail","*")
      .exclude("jcharts","*")
      .exclude("junit","*")
      .exclude("logkit","*")
      .exclude("net.sf.jtidy","*")
      .exclude("org.apache.geronimo.specs","*")
      .exclude("org.apache.httpcomponents","*")
      //.exclude("org.apache.jmeter","jorphan")
      .exclude("org.apache.tika","*")
      .exclude("org.apache.xmlgraphics","*")
      .exclude("org.beanshell","*")
      .exclude("org.bouncycastle","*")
      .exclude("org.htmlparser","*")
      .exclude("org.jsoup","*")
      .exclude("org.mongodb","*")
      .exclude("org.mozilla","*")
      .exclude("xml-apis","*")
      .exclude("xpp3","*")
      .exclude("xmlpull","*")
      .exclude("org.jdom","*")
      .exclude("org.jodd","*")
      .exclude("oro","*")
      .exclude("soap","*")
      .exclude("xalan","*")
      .exclude("xerces","*")
      .exclude("org.slf4j","*")

    // jmeter has dependencies on the older non org.apache.commons. groupId/ commons-pool2#commons-pool2;2.3
    // so we will explicitly bring them in
    //    val commonsMath3 = "commons-math3" % "commons-math3" % "3.4.1" from "http://central.maven.org/maven2/org/apache/commons/commons-math3/3.4.1/commons-math3-3.4.1.jar"
    ///    val commonsPool2 = "commons-pool2" % "commons-pool2" % "2.3" from "http://central.maven.org/maven2/org/apache/commons/commons-pool2/2.3/commons-pool2-2.3.jar"
  }

  val scalastm = "org.scala-stm" %% "scala-stm" % "0.11"

  // for the Groovy API for others
  val groovy = "org.codehaus.groovy" % "groovy-all" % "3.0"

  val scalacsv = "com.github.tototoshi" %% "scala-csv" % "1.3"

  val rsyntaxtextarea = "com.fifesoft" % "rsyntaxtextarea" % "3.1"

  //
  // Logging
  //

  object logging {
    val logback        = "ch.qos.logback"      % "logback-classic"               % "1.2"
    val slf4jJcl       = "org.slf4j"           % "jcl-over-slf4j"                % "1.7"
    val slf4j          = "org.slf4j"           % "slf4j-api"                     % "1.7"
  }

  //val twitterUtilEval = "com.twitter" %% "util-eval" % "6.23.0"

  //
  // Test Dependencies
  //
  object testing {
    val scalatest       = "org.scalatest"      %% "scalatest"    % "3.2"
    val scalacheck      = "org.scalacheck"     %% "scalacheck"   % "1.15"
    val scalamock       = "org.scalamock"      %% "scalamock-scalatest-support" % "5.1"
  }

}
