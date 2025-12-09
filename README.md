# CO2 Emission Calculation Service

Микросервис для расчета выбросов CO2 на основе входных параметров и констант этапов.

## Функциональность

- Расчет выбросов CO2 по формуле: Σ(input_field[i] * stage_constant[i])
- Аутентификация по токену
- Задержка 5-10 секунд для имитации сложных вычислений

## API Endpoints

### POST /calculateEmission

**Request:**
```json
{
    "id": integer,
    "auth_token": string,
    "input_fields": float[],
    "stage_constants": float[]
}
