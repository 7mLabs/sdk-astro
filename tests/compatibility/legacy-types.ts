/** Existing 1.0.2 invocation styles must compile without adding any new result type. */
import {calculate,calculateFromLocal,calculateWithContext,Request,LocalCalculationRequest,Result,CoupleResult,CompositeResult,EventsResult,ForecastResult,QueryResult,CalculationResultFor} from '../../bindings/node';
export function dynamic(request: Request): Result | CoupleResult | CompositeResult | EventsResult | ForecastResult | QueryResult { return calculate(request); }
export function dynamicContext<R extends Request>(request:R): CalculationResultFor<R> { return calculateWithContext(request).result; }
export function local(request:LocalCalculationRequest) {
 switch(request.operation) {
  case 'natal': case 'natalDomains': case 'couple': case 'composite': case 'forecast': case 'query': return calculateFromLocal(request);
  default: { const unreachable:never=request; return unreachable; }
 }
}

// Existing utility-type based consumers retain their old argument/result unions.
type OldReturn = Result | CoupleResult | CompositeResult | EventsResult | ForecastResult | QueryResult;
const oldReturn:OldReturn = {} as ReturnType<typeof calculate>;
const oldParameter:Request = {} as Parameters<typeof calculate>[0];
const oldLocalParameter:LocalCalculationRequest = {} as Parameters<typeof calculateFromLocal>[0];
const oldContextParameter:Request = {} as Parameters<typeof calculateWithContext>[0];
void [oldReturn,oldParameter,oldLocalParameter,oldContextParameter];
