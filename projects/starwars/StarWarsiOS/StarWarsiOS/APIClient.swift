//
// Created by Dylan Wang on 2020/7/1.
// Copyright © 2020 Xingbin Wang. all rights reserved.
// 

import Foundation

class APIClient {
    private let inner: OpaquePointer = create_swapi_client()

    deinit {
        free_swapi_client(inner)
    }

    func loadAllPeople(completion: @escaping (Result<[PeopleNative], Error>) -> Void) {
        let context = CallbackContext(completion: completion)

        let callback = PeopleCallback(owner: Unmanaged.passRetained(context).toOpaque(), onSuccess: { (contextPtr, resPtr) in
            guard let contextPtr = contextPtr, let res = resPtr else { return }
            let peopleWrapper = res.pointee
            let people = UnsafeMutableBufferPointer(start: peopleWrapper.array, count: Int(peopleWrapper.length)).map { $0 }
            let context: CallbackContext = Unmanaged.fromOpaque(contextPtr).takeRetainedValue()
            context.completion(.success(people))
        }) { (errPtr) in
            guard let ptr = errPtr else { return }
            print("Error callback")
        }
        load_all_people(inner, callback)
    }
}

class CallbackContext {
    var completion: ((Result<[PeopleNative], Error>) -> Void)
    init(completion: @escaping (Result<[PeopleNative], Error>) -> Void) {
        self.completion = completion
    }
}
