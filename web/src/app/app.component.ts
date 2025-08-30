import { AfterViewInit, Component, ElementRef, OnInit, ViewChild } from '@angular/core';

import { find_function } from 'eureka-finder-native';
import { render } from 'katex';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  operations = [
    { value: '+', label: 'Sum' },
    { value: '-', label: 'Subtraction' },
    { value: '*', label: 'Multiplication' },
    { value: '/', label: 'Division' },
    { value: '^', label: 'Power' },
    { value: 'neg', label: 'Negation' },
    { value: 'sqrt', label: 'Square Root' },
  ];

  selectedOps: string[] = this.operations.map(i => i.value);

  inputs: string = "1-9"; 
  objective: number = 3.14159265359; 
  correctDigits: number = 4; 
  maxSeconds: number = 15; 

  @ViewChild('latex') latex!: ElementRef; 
  @ViewChild('result') result!: ElementRef; 
  @ViewChild('distance') distance!: ElementRef; 

  loading: boolean = false;

  resetToSample() {
    this.inputs = "1-9"; 
    this.objective = 3.14159265359; 
    this.correctDigits = 4; 
    this.maxSeconds = 15; 
  }

  findOperation() {
    let inputs = this.selectedOps.join(",") + "," + this.inputs;

    this.loading = true;

    setTimeout(() => {
      find_function(
        inputs, this.objective, 
        Math.pow(10, -Math.floor(this.correctDigits)), Math.floor(this.maxSeconds),
        (op: string, result: string, dist: number) => {
          render(op, this.latex.nativeElement)
          render(result, this.result.nativeElement)
          render(dist.toString(), this.distance.nativeElement)
        }
      )

      this.loading = false;
    }, 1000)
  }
}
