package org.inosion.dadagen.support.gatling

import akka.actor.{Props, ActorRef}
import io.gatling.core.Predef._
import io.gatling.http.Predef._
import io.gatling.core.action.{Chainable, Failable}
import io.gatling.core.action.builder.ActionBuilder
import io.gatling.core.config.Protocols
import io.gatling.core.feeder.FeederWrapper
import io.gatling.core.result.writer.DataWriter
import io.gatling.core.validation.{Failure, Validation}

class DoNothingAction(val next: ActorRef) extends Chainable with Failable {

  override def executeOrFail(session: Session): Validation[_] = {
      session.attributes.get("message") match {
        case None => Failure("missing the message from the dadagen")
        case Some(msg) => DataWriter.dispatch(session.scenarioName, session.userId, msg)
      }
  }

}

class ExampleGatlingSimulation extends Simulation {

  def doNothing() = new ActionBuilder {
    override def build(next: ActorRef, protocols: Protocols) = system.actorOf(Props(new DoNothingAction(next)))
  }

  import org.inosion.dadagen.api.scaladsl._

  val feeder = dadagen asMaps {
     field { "id".rownumber }
     .field { "gender".gender }
     .field { "firstname".name firstname }
     .field { "surname".name surname }
     // Combine all the values together .. order (what it depends on) does not matter
     .field { "message".template("${id} - ${firstname} ${surname} (${gender}) i:${int} ${ref}")}
     .field { "int".number between 10 and 99876 }
     .field { "ref".regexgen("[a-f]{6}-[0-9a-f]{8}") }
  } generate() // call generate to make the Iterator

  val scn = scenario("Actor Gets a Message").feed(FeederWrapper(feeder)).exec(doNothing)

  setUp(scn.inject(atOnceUsers(10)))

}
