package org.inosion.dadagen.ui

import java.nio.file.{Paths, Files}
import java.text.SimpleDateFormat
import java.util.Calendar
import javafx.beans.value
// import javafx.embed.swing.SwingNode
import javafx.scene.control


// import scalafx.Includes._
// import scalafx.application.JFXApp
// import scalafx.scene.Scene
// import scalafx.scene.paint.Color._
// import scalafx.scene.shape.Rectangle

import org.fife.ui.rsyntaxtextarea.{SyntaxConstants, RSyntaxTextArea}
import org.fife.ui.rtextarea.RTextScrollPane
import org.inosion.dadagen.support.ScalaScriptEngine
import org.inosion.dadagen.lists.{ListConfigSupport, ListManager}

import scala.concurrent.Future
import scala.util.{Success, Failure}
import scalafx.application.{Platform, JFXApp}
import scalafx.application.JFXApp.PrimaryStage
import scalafx.event.ActionEvent
import scalafx.geometry.{Pos, Insets}
import scalafx.scene.Scene
import scalafx.scene.control._
import scalafx.scene.layout._
import scalafx.scene.paint.Color._
import scalafx.scene.shape.Rectangle
import scalafx.stage.{Screen, FileChooser}
import scalafx.Includes._
import scala.concurrent.ExecutionContext.Implicits.global

object DadagenUi extends JFXApp {

  val PotatoList = "miscellaneous.potato-varieties"
  val ColourList = "miscellaneous.colours"

  val fxec = JavaFXExecutionContext.javaFxExecutionContext

  ListConfigSupport.importConfigListData(PotatoList)
  ListConfigSupport.importConfigListData(ColourList)

  // load the script engine (bootstrap SecureRandom).. in the background so it's ready
  val f = Future { ScalaScriptEngine.loadEngine.eval("1") }
  f onComplete { 
    case Success(engine) => goodMessage("Scala DSL compiler is ready") 
    case Failure(engine) => badMessage("Problems were encountered loading the Compiler")
  }

  val Padding = 10
  val DefaultWidth = 800
  val DefaultHeight = 770
  val DefaultRows = 35
  val DefaultCols = 102
  val SampleConfig =
    """
      |  // Sample Dadagen Configuration
      |  //
      |  // This import allows us to use the Case Class directly.
      |  // field { DoubleGenerator("initial_investment",1000,80000,2) }.
      |
      |  import org.inosion.dadagen.generators._
      |
      |  field { "id".rownumber }.
      |  field { "r_uuid".regexgen ("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}") } .
      |  field { "r_rand1".number between 10000 and 90000 }.
      |  field { "r_str".regexgen ("[A-Z][a-zA-Z]{4}[0-9]{4}") }.
      |  field { "payload_id".template ("PERFT_${id}_${r_uuid}") }.
      |  field { "gender".gender }.
      |  field { "firstname".name firstname }.
      |  field { "surname_data".name surname }.
      |  field { "surname".template ("${surname_data}-${r_str}") }.
      |  field { "fullname".template ("${firstname} ${surname}") }.
      |  field { "dob".regexgen ("19[3-9][0-9]-(1[012]|0[1-9])-(0[0-9]|1[0-9]|2[0-9])") }.
      |  field { "email_address".template("TEST_${firstname}.${surname}@noemail.test") }.
      |  field { "nino".regexgen("([AEHKLTYZ][ABEHK-MPR-TW-Y]|[AEHKLTY][Z]|[B][ABEHK-MT]|[C][ABEHKLR]|[G][Y]|[J][A-CEGHJ-NPR-TW-Z]|[M][AWX]|[N][ABEHLMPRSW-Z]|[O][ABEHK-MPRSX]|[P][A-CEGHJ-NPR-TW-Y]|[R][ABEHKMPR-TW-Z]|[S][A-CEGHJ-NPR-TW-Z]|[W][ABEK-MP])[0-9]{6}[A-D ]") }.
      |  field { "street_number".number between 1 and 100 }.
      |  field { "street_name".template ("RS Performance Street" ) }.
      |  field { "town".address city }.
      |  field { "postcode".regexgen ("[A-Z][A-Z][0-9] [0-9][A-Z][A-Z]") }.
      |  // Issue #10 - API is not supporting precision right now, but the case class does (the default _is_ 2.. see "upfront_commission") .. but if you want more precision, this is how
      |  field { DoubleGenerator("initial_investment",1000,80000,2) }.
      |  field { "regular_investment_amount".regexgen("(50|100|150|200|250|300|350|400|450|500|550|600|650|700|750|800|850|900|950)") }.
      |  field { "account_number".number between 8800000 and 8899999 }.
      |  field { "sort_code".regexgen("(402205|110124|830608|880011|938424|938343|938130)") }.
      |  field { "mobile_phone_number".regexgen ("07777 [0-9]{3} [0-9]{3}") }.
      |  field { "retirement_age".number between 65 and 75 }.
      |  field { "upfront_commission".number between 100.00 and 150.00 }. // when using floats, the default is precision 2 (that is, this will create eg 110.18 )
      |  field { "commission_percentage".number between 0.01 and 0.05 }
      |  """.stripMargin

  stage = new PrimaryStage {
    title = "Dadagen"
    scene = new Scene {
      fill = LightGoldrenrodYellow
      content = mainPane
      stylesheets = List( DadagenUi.getClass.getClassLoader.getResource("application.css").toExternalForm )
    }
  }

  def calcWidth = {
    val bounds = Screen.primary.visualBounds
    bounds.width / 2
  }

  def calcHeight = {
    val bounds = Screen.primary.visualBounds
    bounds.height / 2
  }

  def editorTextArea = rsyntaxArea.getContent.asInstanceOf[RTextScrollPane].getTextArea

  /*
   * Main panel
   */
  def mainPane = new BorderPane {
    padding = Insets(Padding)
    left = new Region {
      minHeight = editorTextArea.getLineCount * editorTextArea.getLineHeight
    }
    center = rsyntaxArea
    bottom = bottomPane
  }

  /*
   * Main Editing Panel - Create the rsyntax Swing Node
   */
  lazy val rsyntaxArea = {

    val textArea = new RSyntaxTextArea(DefaultRows, DefaultCols)
    textArea.setSyntaxEditingStyle(SyntaxConstants.SYNTAX_STYLE_SCALA)
    textArea.setCodeFoldingEnabled(true)
    textArea.setText(SampleConfig)

    val s = new scalafx.embed.swing.SwingNode()
    s.setContent(new RTextScrollPane(textArea))
    s

  }

  /*
   * Bottom on window
   */
  lazy val bottomPane = new VBox {
    children = Seq(
      messagesPane,
      new HBox {
        children = Seq(fileNameField, /* need A Spacer here */ browseButton)
      },
      new HBox {
        padding = Insets(Padding)
        children = Seq(
          numberLabel
          , numberOf
          , fileTypeRadios
          , generateButton
        )
        hgrow = Priority.Always
        alignment = Pos.CenterRight
      }
    )
  }

  lazy val fileChooser = new FileChooser()

  lazy val browseButton = new Button("Browse ...") {
    onAction = (e: ActionEvent) => {
      val f = fileChooser.showSaveDialog(stage)
      if (f != null) {
        fileNameField.text = f.getAbsolutePath
      }
    }
  }

  // TODO change to TextFlow
  lazy val messages = new TextArea {
    prefRowCount = 5
    styleClass = "messages" :: Nil
  }

  lazy val messagesPane = new ScrollPane {
    content = messages
    disable = false
    fitToWidth = true
    fitToHeight = true
  }


  /*
   * File Output Type Selectors (buttons)
   */

  lazy val fileTypeGroup = {
    val tg = new ToggleGroup()

    // we are going to make the toggle button "behave" like radio buttons
    tg.selectedToggleProperty().addListener(
      (observable: value.ObservableValue[_ <: control.Toggle], oldValue: control.Toggle, newValue: control.Toggle) => {
        if (newValue == null) {
          Platform.runLater({
            tg.selectToggle(oldValue)
          })
        }

        setTheFileTypeExtension(newValue.getUserData.asInstanceOf[FileType])
    })
    tg
  }

  lazy val jsonButton = new ToggleButton {
    text = JsonFile.toString
    userData = JsonFile
    toggleGroup = fileTypeGroup
  }
  lazy val csvButton = new ToggleButton {
    text = CSVFile.toString
    userData = CSVFile
    selected = true
    toggleGroup = fileTypeGroup
  }
  lazy val xmlButton = new ToggleButton {
    text = XmlFile.toString
    userData = XmlFile
    toggleGroup = fileTypeGroup
  }

  lazy val fileTypeRadios = {

    val gp = new GridPane {
      hgap = 10
      vgap = 1
      alignment = Pos.Center
    }

    gp.add(jsonButton,0,0)
    gp.add(csvButton,1,0)
    gp.add(xmlButton,2,0)

    gp
  }

  def setTheFileTypeExtension(fileType: FileType) = {
    fileNameField.setText(
       fileNameField.getText.replaceAll("\\.[^.]*$", "") + "." + fileType.extension
    )
  }

  // some other file controls

  lazy val numberLabel = new Label("Number of rows / samples")

  lazy val numberOf = new TextField{
    prefColumnCount = 15
    text = "100"
  }

  /*
   * Determine the currently selected "type"
   */
  def selectedType() = fileTypeGroup.selectedToggle().getUserData.asInstanceOf[FileType]

  private val timeFmt = new SimpleDateFormat("dd MMM HH:mm:ss ")

  def updateMessage(msg:String, good:String) = {
    System.out.println(msg)
    val prefix       = if ("".equals(messages.text)) "" else messages.text + "\n"
    val goodBadPrefx = if ("true".equals(good)) "\u26AA " else "\u25B6 "
    val timestamp    = timeFmt.format(Calendar.getInstance().getTime)
    messages.text    = prefix + timestamp + goodBadPrefx + msg
  }

  // wowo I hate implicits, I can't work out how to turn off a Boolean to scalafx.beans.binding.BooleanBinding implicit
  // wo weaving arount it my changing my method to string :ROFL: :-(
  def goodMessage = updateMessage(_:String,good="true")
  def badMessage = updateMessage(_:String,good="false")

  lazy val generateButton = new Button("Generate Data") {
    onAction = (me: ActionEvent) => {
      val b = me.source.asInstanceOf[javafx.scene.control.Button]
      val f = Future {
        b.disable = true
        DadageneratorHelper.generateData(numberOf.getText.toInt
          , selectedType()
          , fileNameField.getText
          , rsyntaxArea.getContent().asInstanceOf[RTextScrollPane].getTextArea.getText)
      }
      f onComplete { 
        case Failure(error) => {
          badMessage(error.getLocalizedMessage)
        }
        case Success(message) => {
          goodMessage(message)
          optionallySetNewRandomFileName()
          b.disable = false
        }
      }
    }
  }

  lazy val fileNameField = new TextField {
    text = FileSupport.fullFileRandomPrefix + FileSupport.randomSuffix + ".csv"
    minWidth = 700
  }

  def optionallySetNewRandomFileName() : Unit = {
    val TempFileMatch = s"^(${FileSupport.fullFileRandomPrefix}).*".r
    fileNameField.getText match {
      case TempFileMatch(e) => fileNameField.setText(FileSupport.fullFileRandomPrefix + FileSupport.randomSuffix + "." + selectedType().extension)
      case _ => // ignore changing the field -- the user has changed it to their OWN filepath
    }
  }

}

object FileSupport {

  implicit val rand = new java.security.SecureRandom()

  /*
   * For initial launch, randomise a filename
   */
  val RandomFileName = "Dadagen-" + ListManager.getRandomValue("miscellaneous.potato-varieties")

  def fullFileRandomPrefix = Paths.get(defaultDirectory.toString, RandomFileName)

  def defaultDirectory = {
    val docPath = Paths.get(System.getProperty("user.home"),"Documents")
    if (Files.exists(docPath)) {
      docPath
    } else {
      Files.createTempDirectory("_dadagen").getFileName
    }
  }

  def randomSuffix = "-" + System.currentTimeMillis().toString.substring(4)

}