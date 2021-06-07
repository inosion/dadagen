package org.inosion.dadagen

import org.inosion.dadagen.generators._
import org.junit.runner.RunWith
import org.scalatestplus.junit.JUnitRunner
import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers

@RunWith(classOf[JUnitRunner])
class NamesGeneratorTest extends AnyFlatSpec with Matchers {

  val femaleTitles = RadgContext.Settings.getList("dadagen.lists.person.title.values.F")
  val maleTitles = RadgContext.Settings.getList("dadagen.lists.person.title.values.M")

  val regexp = """^(.*)\s+(.*)\s+(.*)$""".r

  "Generating a full name" should "match a regular expression" in {
    import org.inosion.dadagen.generators._
    val ramondom = DadagenStringList(List(
      FullNameGenerator("full_name")
    ))

    val result = ramondom.generateAll(9000)

    result.length should be (9000)

    for (row <- result) {
      regexp.findFirstIn(row(0)).isDefined should be (true)
    }

  }
}

