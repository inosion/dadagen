
package org.inosion.dadagen

import org.junit.runner.RunWith
import org.scalatestplus.junit.JUnitRunner
import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers

import org.inosion.dadagen.generators._

@RunWith(classOf[JUnitRunner])
class TitleAndGenderTest extends AnyFlatSpec with Matchers {

  val femaleTitles = RadgContext.Settings.getList("dadagen.lists.person.title.values.F")
  val maleTitles = RadgContext.Settings.getList("dadagen.lists.person.title.values.M")
  "Title and Gender bound" should "always match the correct title for gender" in {

    import org.inosion.dadagen.generators._
    val ramondom = DadagenStringList(List(
      TitleGenerator("title",Some("gender")),
      GenderGenerator("gender")
    ))

    val result = ramondom.generateAll(200)
    for (row <- result) {
      List("M","F") should contain (row(1))
      row(1) match {
        case "F" => femaleTitles should contain(row(0))
        case "M" => maleTitles should contain(row(0))
      }
    }
  }
}

