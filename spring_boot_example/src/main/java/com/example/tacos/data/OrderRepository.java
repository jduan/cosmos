package com.example.tacos.data;

import com.example.tacos.Order;

public interface OrderRepository {
  Order save(Order order);
}
