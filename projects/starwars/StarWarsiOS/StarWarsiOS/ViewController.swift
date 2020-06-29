//
// Created by Dylan Wang on 2020/7/1.
// Copyright © 2020 Xingbin Wang. all rights reserved.
// 

import UIKit

class ViewController: UIViewController {

    let client = APIClient()

    override func viewDidLoad() {
        super.viewDidLoad()

        client.loadAllPeople { (res) in
            switch res {
            case .success(let people):
                for person in people {
                    if let name = person.gender {
                        let string = String(cString: name)
                        print(string)
                    }
                }
            case .failure(let e): break
            }
        }
    }


}

