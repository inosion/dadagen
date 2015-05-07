
  package org.inosion.datdom

  import org.scalatest.{Matchers, FlatSpec}
  import org.inosion.datdom.randomtypes._


class TitleAndGenderTest extends FlatSpec with Matchers {

  val femaleTitles = RadgContext.Settings.getList("datdom.lists.person.title.values.F")
  val maleTitles = RadgContext.Settings.getList("datdom.lists.person.title.values.M")
  "Title and Gender bound" should "always match the correct title for gender" in {
    val ramondom = SeqOfSeqOfStringsGenerator(List(
      TitleGenerator("title",Some("gender")),
      GenderGenerator("gender")
    ))

    val result = ramondom.generateAll(200)
    for (row <- result._2) {
      List("M","F") should contain (row(1))
      row(1) match {
        case "F" => femaleTitles should contain(row(0))
        case "M" => maleTitles should contain(row(0))
      }
    }
  }
}

