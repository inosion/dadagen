package org.inosion.dadagen

import org.scalatest.{FlatSpec, Matchers}


  class NullListCheckTest extends FlatSpec with Matchers {

    "A null Generator List" should "throw a require error" in {
      try {
        val gen = MapOfStringsGenerator(null)
        fail("did not error when null list provided")
      } catch {
        case _: IllegalArgumentException => // expected
      }
    }
  }

