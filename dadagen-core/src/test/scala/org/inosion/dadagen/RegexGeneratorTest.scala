package org.inosion.dadagen

import org.inosion.dadagen.generators.{RegexGenerator, GenderGenerator, TitleGenerator}
import org.junit.runner.RunWith
import org.scalatestplus.junit.JUnitRunner
import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers

/**
 * @author rbuckland
 */
@RunWith(classOf[JUnitRunner])
class RegexGeneratorTest extends AnyFlatSpec with Matchers {
  "Supplying a Regular Expression" should "generate data that matches" in {
    val emailRegexp = """my\.test\.email\.[a-f]{6}\.[0-9]{10}\@foo\.com"""
    val ramondom = DadagenStringList(List(
      TitleGenerator("title",Some("gender")),
      GenderGenerator("gender"),
      RegexGenerator("email_address",emailRegexp)
    ))

    val result = ramondom.generateAll(100)
    for (row <- result) {
      // look in the 3rd element at the email. :: eg my.test.email.ddbcda.4828430423@foo.com
      emailRegexp.r.findFirstIn(row(2)).isDefined should be (true)
    }
    println("regexp generator --> " + result.head)
  }
}

