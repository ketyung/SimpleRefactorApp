//
//  ContentView.swift
//  SimpleRefactorApp
//
//  Created by Christopher Chee on 28/01/2022.
//

import SwiftUI

struct ContentView: View {
    
    
    let array = ["One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight"]
    
    var body: some View {
       
        ForEach(Array(array.enumerated()), id: \.offset){ index, str  in
            
            geoView(str: str, index: index)
        }
    
    }
}


extension ContentView {
   
    func geoView(str : String, index : Int)-> some View {
        
        GeometryReader { geo in
            
            ifView(geo: geo, str: str, index: index)
        }
        .padding(8)
        .frame(width: UIScreen.main.bounds.width * 0.9, height: 60, alignment: .center)
        .background(Color.black)
        .cornerRadius(30)
        
    }
}

extension ContentView {
    
    @ViewBuilder
    func ifView (geo : GeometryProxy, str : String, index : Int) -> some View {
        
        
        if ( index % 2 == 0){
            
            view1(geo: geo, str: str, index: index)
        
        }
        else {
            
            view2(geo: geo, str: str, index: index)
        }
    }
}

extension ContentView {
    
    func view1(geo : GeometryProxy, str : String, index : Int) -> some View {
        
        Text("\(str)")
        .frame(width:geo.size.width * 0.8, height:geo.size.height * 0.65)
        .position(x: geo.size.width / 2, y: geo.size.height / 2)
        .foregroundColor(.white)
        .background(Color.blue)
        .cornerRadius(30)
        
    }
    
    func view2(geo : GeometryProxy, str : String, index : Int) -> some View{
        
        HStack (spacing:20) {
            
            ZStack{
                
            
                Circle().fill(Color.red).frame(width: 30, height: 30)
                
                Text("\(index+1)")
                
            }
            
            Text("\(str)")
        }
        .frame(width:geo.size.width * 0.8, height:geo.size.height * 0.65)
        .position(x: geo.size.width / 2, y: geo.size.height / 2)
        .foregroundColor(.white)
        .background(Color.green)
        .cornerRadius(30)
    }
}


struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
