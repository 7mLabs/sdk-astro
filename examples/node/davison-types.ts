import { calculate, calculateFromLocal, DavisonRequest, DavisonResult, CalculationRequest } from '../../bindings/node';
const birth = { utc: { year: 2000, month: 1, day: 1, hour: 12, minute: 0 }, location: { latitude: 10, longitude: 106 } };
const r: DavisonRequest = { operation: 'davison', variant: 'corrected', personA: birth, personB: birth };
const result: DavisonResult = calculate(r);
const variant: 'uncorrected' | 'corrected' = result.calculation.variant;
const reportKind: 'davisonRelationship' | undefined = result.data.domains.love?.report.chartKind;
const request: CalculationRequest = r;
const local: DavisonResult = calculateFromLocal({ operation: 'davison', variant: 'uncorrected', personA: { local: birth.utc, timeZone: 'Asia/Ho_Chi_Minh', location: birth.location }, personB: birth });
// @ts-expect-error Davison is its own operation, not a symbolic composite house method.
const wrong: DavisonRequest = { ...r, houseMethod: 'midpoint' };
// @ts-expect-error Invalid variant is not accepted.
const wrongVariant: DavisonRequest = { ...r, variant: 'spherical' };
void [variant, reportKind, request, local, wrong, wrongVariant];
