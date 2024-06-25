//
//  ViewController.swift
//
//  Created by LiJinlei on 2021/9/10.
//

import UIKit

class ViewController: UIViewController {
    @IBOutlet var metalV: MetalView!
    var wgpuCanvas: OpaquePointer?
    
    lazy var displayLink: CADisplayLink = {
        CADisplayLink.init(target: self, selector: #selector(enterFrame))
    }()
    
    override func viewDidLoad() {
        super.viewDidLoad()
       
        self.displayLink.add(to: .current, forMode: .default)
        self.displayLink.isPaused = true
    }
    
    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        self.view.backgroundColor = .white
        if wgpuCanvas == nil {
            let viewPointer = Unmanaged.passUnretained(self.metalV).toOpaque()
            let metalLayer = Unmanaged.passUnretained(self.metalV.layer).toOpaque()
            let maximumFrames = UIScreen.main.maximumFramesPerSecond
            
            let viewObj = ios_view_obj(view: viewPointer, metal_layer: metalLayer,maximum_frames: Int32(maximumFrames), callback_to_swift: callback_to_swift)
            
            wgpuCanvas = create_wgpu_canvas(viewObj)
        }
        self.displayLink.isPaused = false
    }
    
    override func viewWillDisappear(_ animated: Bool) {
        super.viewWillDisappear(animated)
        displayLink.isPaused = true
    }
    
    @objc func enterFrame() {
        guard let canvas = self.wgpuCanvas else {
            return
        }
        // call rust
        enter_frame(canvas)
    }
    
    @IBAction func changeExample(sender: UISegmentedControl) {
        guard let canvas = self.wgpuCanvas else {
            return
        }
        var index = sender.selectedSegmentIndex
        if index == 2 {
            index = 5
        }
        change_example(canvas, Int32(index))
    }
    
    @objc func touchE(x: Float32, y: Float32, phase: Int32) {
        guard let canvas = self.wgpuCanvas else {
            return
        }
        // call rust
        touch(canvas, x, y, phase);
    }
    
    // Override touch event methods
    override func touchesBegan(_ touches: Set<UITouch>, with event: UIEvent?) {
        super.touchesBegan(touches, with: event)
        handleTouch(touches)
    }
    
    override func touchesMoved(_ touches: Set<UITouch>, with event: UIEvent?) {
        super.touchesMoved(touches, with: event)
        handleTouch(touches)
    }
    
    override func touchesEnded(_ touches: Set<UITouch>, with event: UIEvent?) {
        super.touchesEnded(touches, with: event)
        handleTouch(touches)
    }
    
    override func touchesCancelled(_ touches: Set<UITouch>, with event: UIEvent?) {
        super.touchesCancelled(touches, with: event)
        handleTouch(touches)
    }
    
    
    // Define your handleTouch method
    func handleTouch(_ touches: Set<UITouch>) {
        // Implement your touch handling logic here
        for touch in touches {
            let location = touch.location(in: self.view)
            //print("Touch at location: \(location)")
            // You can add more logic here as needed
            var phase = 0;
            if touch.phase == UITouch.Phase.began {
                phase = 0;
            }
            if touch.phase == UITouch.Phase.moved {
                phase = 1;
            }
            if touch.phase == UITouch.Phase.ended {
                phase = 2;
            }
            if touch.phase == UITouch.Phase.cancelled {
                phase = 3;
            }
            touchE(x: Float(location.x), y: Float(location.y), phase: Int32(phase));
        }
    }
    

}

func callback_to_swift(arg: Int32) {
    DispatchQueue.main.async {
        switch arg {
        case 0:
            print("wgpu canvas created!")
            break
        case 1:
            print("canvas enter frame")
            break
            
        default:
            break
        }
    }
    
}
