package io.straight.radg

import io.straight.radg.randomtypes.{RegexGenerator, GenderGenerator, TitleGenerator}
import org.scalatest.{Matchers, FlatSpec}

/**
 * @author rbuckland
 */
class RegexGeneratorTest extends FlatSpec with Matchers {
  "Title and Gender bound" should "always match the correct title for gender" in {
    val ramondom = SeqOfSeqOfStringsGenerator(100,List(
      TitleGenerator("title",Some("gender")),
      GenderGenerator("gender"),
      RegexGenerator("email_address","""my\.test\.email\.[a-f]{6}\.[0-9]{10}\@foo\.com""")
    ))

    val result = ramondom.generate()
    for (row <- result._2) {
      println(row)
    }
  }
}

