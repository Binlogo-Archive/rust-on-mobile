#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "starwars_core.h"

typedef struct {
  const char *name;
  const char *gender;
  const char *mass;
} PeopleNative;

typedef struct {
  PeopleNative *array;
  uintptr_t length;
} PeopleNativeWrapper;

typedef struct {
  void *owner;
  void (*onSuccess)(void *owner, const PeopleNativeWrapper *res);
  void (*onError)(void *owner);
} PeopleCallback;

SwapiClient *create_swapi_client(void);

void free_swapi_client(SwapiClient *client);

void load_all_people(SwapiClient *client, PeopleCallback callback);
