package org.soqqo.datagen;

import java.util.Date;

public class Person {

    private String firstname;
    private String lastname;
    private Age myAge;
    private Date created;
    public String getFirstname() {
        return firstname;
    }
    public void setFirstname(String firstname) {
        this.firstname = firstname;
    }
    public String getLastname() {
        return lastname;
    }
    public void setLastname(String lastname) {
        this.lastname = lastname;
    }
    
    public String toString() { 
        return lastname + ", " + firstname;
    }
    public Age getMyAge() {
        return myAge;
    }
    public void setMyAge(Age myAge) {
        this.myAge = myAge;
    }
    public Date getCreated() {
        return created;
    }
    public void setCreated(Date created) {
        this.created = created;
    }

}
