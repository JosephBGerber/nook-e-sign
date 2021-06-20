package com.github.josephbgerber.nookesign;

public class Status {
    public int id;
    public int charge;
    public String image_hash;

    @Override
    public String toString() {
        return "Status{" +
                "id=" + id +
                ", charge=" + charge +
                ", image_hash='" + image_hash + '\'' +
                '}';
    }
}
