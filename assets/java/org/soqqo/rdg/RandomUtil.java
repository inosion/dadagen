package org.soqqo.rdg;

import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.security.SecureRandom;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;

import org.soqqo.rdg.config.DataTypes;

public class RandomUtil {

    private static final String LASTNAMES_CSV = "lastnames.csv";

    private static final String FIRSTNAMES_CSV = "firstnames.csv";

    public enum ListType {
        Lastname, MaleFirstname, FemaleFirstname, Firstname;
    }

    private static SecureRandom rand = new SecureRandom();

    private static HashMap<ListType, ArrayList<String>> listCache = new HashMap<ListType, ArrayList<String>>();

    /**
     * Select a random name from a list.
     * 
     * @param DataTypes
     *            .Name
     *            nameType
     * @return String
     *         a Single random Name
     */
    public static String randomName(DataTypes.Name nameType) {
        ArrayList<String> names = list(ListType.valueOf(nameType.name()));
        return names.get(rand.nextInt(names.size()));
    }

    private static ArrayList<String> list(ListType listType) {
        if (!listCache.containsKey(listType)) {
            listCache.put(listType, new ArrayList<String>());
            loadFile(listType, listCache.get(listType));
        }
        return listCache.get(listType);
    }

    private static void loadFile(ListType listType, ArrayList<String> arrayList) {
        switch (listType) {
        case Firstname: {
            readAsStrings(arrayList, FIRSTNAMES_CSV, null);
            break;
        }
        case FemaleFirstname: {
            readAsStrings(arrayList, FIRSTNAMES_CSV, "F");
            break;
        }
        case MaleFirstname: {
            readAsStrings(arrayList, FIRSTNAMES_CSV, "M");
            break;
        }
        case Lastname: {
            readAsStrings(arrayList, LASTNAMES_CSV, null);
            break;
        }
        }
    }

    public static String randomFromStringSeparatedList(String commaSeparatedList) {
        String[] list = commaSeparatedList.split(",");
        return list[rand.nextInt(list.length)].trim();
    }

    public static List<String> stringToList(String commaSeparatedList) {
        ArrayList<String> list = new ArrayList<String>();
        String[] preprocessedList = commaSeparatedList.split(",");

        for (String string : preprocessedList) {
            list.add(string.trim());
        }
        return list;
    }

    public static <E> E randomFromList(List<E> aList) {
        return aList.get(rand.nextInt(aList.size()));
    }

    private static void readAsStrings(ArrayList<String> data, String filename, String discriminator) {

        try {
            BufferedReader reader = new BufferedReader(new InputStreamReader(RandomUtil.class.getClassLoader().getResourceAsStream(filename)));
            String nextLine = reader.readLine();
            while (nextLine != null) {
                if (!nextLine.contains(",")) {
                    data.add(nextLine);
                } else {
                    String[] values = nextLine.split(",");
                    if (discriminator == null || discriminator.equals(values[1])) {
                        data.add(values[0]);
                    }
                }
                nextLine = reader.readLine();
            }
            reader.close();
        } catch (Exception e) {
            throw new DataGenException("Failed to read the file : [" + filename.toString() + "]", e);
        }
    }
}
