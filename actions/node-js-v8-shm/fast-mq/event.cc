// The faaas project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Ye

#include <node.h>
#include <v8.h>

using namespace v8;

Handle<Value> Method(const Arguments &args) {
  HandleScope scope;
  return scope.Close(String::New("world"));
}

void init(Handle<Object> target) {
  target->Set(String::NewSymbol("hello"), 
    FunctionTemplate::New(Method)->GetFunction());
}

NODE_MODULE(fast-mq, init)
