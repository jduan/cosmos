package com.example.tacos.data;

import com.example.tacos.Ingredient;

public interface IngredientRepository {
  Iterable<Ingredient> findAll();
  Ingredient findOne(String id);
  Ingredient save(Ingredient ingredient);
}
