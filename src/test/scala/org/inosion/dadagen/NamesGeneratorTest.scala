package org.inosion.dadagen

import org.inosion.dadagen.randomtypes._
import org.scalatest.{FlatSpec, Matchers}

class NamesGeneratorTest extends FlatSpec with Matchers {

  val femaleTitles = RadgContext.Settings.getList("dadagen.lists.person.title.values.F")
  val maleTitles = RadgContext.Settings.getList("dadagen.lists.person.title.values.M")

  val regexp = """^(.*)\s+(.*)\s+(.*)$""".r

  "Generating a full name" should "match a regular expression" in {
    val ramondom = ListOfStringsGenerator(List(
      FullNameGenerator("full_name")
    ))

    val result = ramondom.generateAll(9000)

    result.length should be (9000)

    for (row <- result) {
      regexp.findFirstIn(row(0)).isDefined should be (true)
    }

  }
}

