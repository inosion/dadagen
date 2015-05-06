package org.soqqo.rdg.config;

import java.security.SecureRandom;
import java.util.Calendar;
import java.util.Date;

import org.soqqo.rdg.DataGenException;
import org.soqqo.rdg.config.DataTypes.DTConfig;
import org.soqqo.rdg.config.DataTypes.DateAndTime;

public class DateAndTimeFieldConfig extends DataFieldConfig {

    private Date endDate;
    private Date beginDate;
    private static SecureRandom rand = new SecureRandom();
    private DTConfig restrictor;

    public DateAndTimeFieldConfig(String propertyName, DateAndTime generationType) {
        super(propertyName, generationType);
    }

    public DateAndTimeFieldConfig(String propertyName, DateAndTime generationType, DTConfig restrictor) {
        super(propertyName, generationType);
        this.restrictor = restrictor;
    }

    public DateAndTimeFieldConfig(String propertyName, DateAndTime generationType, Date beginDate, Date endDate) {
        super(propertyName, generationType);
        this.beginDate = beginDate;
        this.endDate = endDate;
    }

    public DateAndTimeFieldConfig(String propertyName, DateAndTime generationType, Date beginDate, Date endDate, DTConfig restrictor) {
        super(propertyName, generationType);
        this.beginDate = beginDate;
        this.endDate = endDate;
        this.restrictor = restrictor;
    }

    public Object getNextValue() {
        Date d = null;
        if (getGenerationType().equals(DateAndTime.Random)) {
            if (endDate == null) {
                endDate = new Date();
            }
            if (beginDate == null) {
                beginDate = new Date(0);
            }

            // calc a random number of milliseconds to add
            long newDateMilli = beginDate.getTime() + ((long) (rand.nextDouble() * (endDate.getTime() - beginDate.getTime())));

            d = new Date(newDateMilli);

        } else if (getGenerationType().equals(DateAndTime.Now)) {

            d = new Date();

        } else if (getGenerationType().equals(DateAndTime.Sequential)) {

            throw new DataGenException("Not yet supported, just have not worked out exactly how I want sequential to work");

        }

        if (restrictor != null) {
            long newMilli = d.getTime();
            Calendar c = Calendar.getInstance();
            switch (restrictor) {
            case RoundToSeconds: {
                newMilli = ((long) (d.getTime() / 1000L)) * 1000L;
                c.setTimeInMillis(newMilli);
                break;
            }
            case RoundToMinute: {
                newMilli = ((long) (d.getTime() / 60000L)) * 60000L;
                c.setTimeInMillis(newMilli);
                break;
            }
            case RoundToHour: {
                newMilli = ((long) (d.getTime() / 3600000L)) * 3600000L;
                c.setTimeInMillis(newMilli);
                break;
            }
            case RoundToDay: {
                c.set(Calendar.YEAR, d.getYear() + 1900);
                c.set(Calendar.MONTH, d.getMonth());
                c.set(Calendar.DAY_OF_MONTH, d.getDate());
                c.set(Calendar.HOUR_OF_DAY, 0);
                c.set(Calendar.MINUTE, 0);
                c.set(Calendar.SECOND, 0);
                break;
            }
            case RoundToMonth: {
                c.set(Calendar.YEAR, d.getYear() + 1900);
                c.set(Calendar.MONTH, d.getMonth());
                c.set(Calendar.DAY_OF_MONTH, 1);
                c.set(Calendar.HOUR_OF_DAY, 0);
                c.set(Calendar.MINUTE, 0);
                c.set(Calendar.SECOND, 0);
                break;
            }
            case RoundToYear: {
                c.set(Calendar.YEAR, d.getYear() + 1900);
                c.set(Calendar.MONTH, 0);
                c.set(Calendar.DAY_OF_MONTH, 1);
                c.set(Calendar.HOUR_OF_DAY, 0);
                c.set(Calendar.MINUTE, 0);
                c.set(Calendar.SECOND, 0);
                break;
            }
            }

            d = c.getTime();

        }
        return d;
    }
}
